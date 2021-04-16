import { Mesh, PerspectiveCamera, BufferGeometry } from 'three';
import { PointerLockControls } from 'three/examples/jsm/controls/PointerLockControls';
import { Coords3, EntityType } from '../libs';
import { Engine } from './engine';
declare type CameraOptionsType = {
    fov: number;
    near: number;
    far: number;
    initPos: [number, number, number];
    minPolarAngle: number;
    maxPolarAngle: number;
    acceleration: number;
    flyingInertia: number;
    reachDistance: number;
    lookBlockScale: number;
    lookBlockLerp: number;
    distToGround: number;
    distToTop: number;
    cameraWidth: number;
};
declare class Camera {
    engine: Engine;
    threeCamera: PerspectiveCamera;
    controls: PointerLockControls;
    options: CameraOptionsType;
    lookBlock: Coords3 | null;
    targetBlock: Coords3 | null;
    camGeometry: BufferGeometry;
    camMesh: Mesh;
    camEntity: EntityType;
    private vec;
    private movements;
    private lookBlockMesh;
    constructor(engine: Engine, options: CameraOptionsType);
    onKeyDown: ({ code }: KeyboardEvent) => void;
    onKeyUp: ({ code }: KeyboardEvent) => void;
    tick: () => void;
    teleport(voxel: Coords3): number[];
    get voxel(): Coords3;
    get position(): Coords3;
    get voxelPositionStr(): string;
    get lookBlockStr(): string;
    private updateLookBlock;
}
export { Camera, CameraOptionsType };
