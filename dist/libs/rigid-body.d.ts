import { AABB } from './aabb';
declare class RigidBody {
    aabb: AABB;
    mass: number;
    friction: number;
    restitution: number;
    gravityMultiplier: number;
    onCollide: (impacts?: number[]) => void;
    autoStep: boolean;
    airDrag: number;
    fluidDrag: number;
    onStep: null | (() => void);
    resting: number[];
    velocity: number[];
    inFluid: boolean;
    ratioInFluid: number;
    forces: number[];
    impulses: number[];
    sleepFrameCount: number;
    constructor(aabb: AABB, mass: number, friction: number, restitution: number, gravityMultiplier: number, onCollide: (impacts?: number[]) => void, autoStep: boolean);
    setPosition(p: number[]): void;
    getPosition(): number[];
    applyForce(f: number[]): void;
    applyImpulse(i: number[]): void;
    markActive(): void;
    get atRestX(): number;
    get atRestY(): number;
    get atRestZ(): number;
}
export { RigidBody };
