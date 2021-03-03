import { Engine } from '..';

import { PerspectiveCamera, Vector3 } from 'three';
import { PointerLockControls } from 'three/examples/jsm/controls/PointerLockControls';
import { Coords3 } from '../libs';
import { Helper } from '../utils';

type CameraOptionsType = {
  fov: number;
  near: number;
  far: number;
  initPos: [number, number, number];
  minPolarAngle: number;
  maxPolarAngle: number;
  acceleration: number;
  flyingInertia: number;
};

const defaultCameraOptions: CameraOptionsType = {
  fov: 75,
  near: 0.1,
  far: 8000,
  initPos: [10, 10, 10],
  minPolarAngle: 0,
  maxPolarAngle: Math.PI,
  acceleration: 1,
  flyingInertia: 3,
};

class Camera {
  public engine: Engine;
  public threeCamera: PerspectiveCamera;
  public controls: PointerLockControls;

  public options: CameraOptionsType;

  private acc = new Vector3();
  private vel = new Vector3();
  private movements = {
    up: false,
    down: false,
    left: false,
    right: false,
    front: false,
    back: false,
  };

  constructor(engine: Engine, options: Partial<CameraOptionsType> = {}) {
    this.options = {
      ...defaultCameraOptions,
      ...options,
    };

    const { fov, near, far, initPos } = this.options;

    this.engine = engine;

    // three.js camera
    this.threeCamera = new PerspectiveCamera(fov, this.engine.rendering.aspectRatio, near, far);

    // three.js pointerlock controls
    this.controls = new PointerLockControls(this.threeCamera, this.engine.container.canvas);
    this.engine.rendering.scene.add(this.controls.getObject());
    this.engine.container.canvas.onclick = () => this.controls.lock();

    this.controls.getObject().position.set(...initPos);

    window.addEventListener('resize', () => {
      engine.container.fitCanvas();
      engine.rendering.adjustRenderer();

      this.threeCamera.aspect = engine.rendering.aspectRatio;
      this.threeCamera.updateProjectionMatrix();
    });

    document.addEventListener('keydown', this.onKeyDown, false);
    document.addEventListener('keyup', this.onKeyUp, false);

    this.threeCamera.lookAt(new Vector3(0, 0, 0));
  }

  onKeyDown = ({ code }: KeyboardEvent) => {
    if (!this.controls.isLocked) return;

    switch (code) {
      case 'ArrowUp':
      case 'KeyW':
        this.movements.front = true;
        break;

      case 'ArrowLeft':
      case 'KeyA':
        this.movements.left = true;
        break;

      case 'ArrowDown':
      case 'KeyS':
        this.movements.back = true;
        break;

      case 'ArrowRight':
      case 'KeyD':
        this.movements.right = true;
        break;

      case 'Space':
        this.movements.up = true;
        break;

      case 'ShiftLeft':
        this.movements.down = true;
        break;
    }
  };

  onKeyUp = ({ code }: KeyboardEvent) => {
    switch (code) {
      case 'ArrowUp':
      case 'KeyW':
        this.movements.front = false;
        break;

      case 'ArrowLeft':
      case 'KeyA':
        this.movements.left = false;
        break;

      case 'ArrowDown':
      case 'KeyS':
        this.movements.back = false;
        break;

      case 'ArrowRight':
      case 'KeyD':
        this.movements.right = false;
        break;

      case 'Space':
        this.movements.up = false;
        break;

      case 'ShiftLeft':
        this.movements.down = false;
        break;
    }
  };

  tick = () => {
    const { delta } = this.engine.clock;

    const { right, left, up, down, front, back } = this.movements;
    const { acceleration, flyingInertia } = this.options;

    const movementVec = new Vector3();
    movementVec.x = Number(right) - Number(left);
    movementVec.z = Number(front) - Number(back);
    movementVec.normalize();

    const yMovement = Number(up) - Number(down);

    this.acc.x = -movementVec.x * acceleration;
    this.acc.y = yMovement * acceleration;
    this.acc.z = -movementVec.z * acceleration;

    this.vel.x -= this.vel.x * flyingInertia * delta;
    this.vel.y -= this.vel.y * flyingInertia * delta;
    this.vel.z -= this.vel.z * flyingInertia * delta;

    this.vel.add(this.acc.multiplyScalar(delta));
    this.acc.set(0, 0, 0);

    this.controls.moveRight(-this.vel.x);
    this.controls.moveForward(-this.vel.z);

    this.controls.getObject().position.y += this.vel.y;
  };

  get voxel(): Coords3 {
    return Helper.vMapWorldPosToVoxelPos(this.position, this.engine.world.options.dimension);
  }

  get position(): Coords3 {
    const { x, y, z } = this.threeCamera.position;
    return [x, y, z];
  }
}

export { Camera };
