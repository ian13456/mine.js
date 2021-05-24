import { Coords3 } from '../../../shared/types';
import { Chunk, Mine, TERRAIN_CONFIG } from '../../core';
import { Noise } from '../noise';
import { VoxelUpdate } from '../types';

import { Base } from './base';

class Trees extends Base {
  constructor() {
    super([5, 5]);
  }

  isTreeLocation(vx: number, vz: number) {
    const noise3x3 = [];

    const {
      HILLY: { TREE_SCALE },
    } = TERRAIN_CONFIG;

    for (let i = -1; i <= 1; i++) {
      for (let j = -1; j <= 1; j++) {
        noise3x3.push(Noise.perlin2(vx + i, vz + j, TREE_SCALE));
      }
    }

    let max = noise3x3[0];
    let maxi = 0;

    for (let i = 1; i < noise3x3.length; i++) {
      if (max < noise3x3[i]) {
        max = noise3x3[i];
        maxi = i;
      }
    }

    return maxi === 4;
  }

  sample(chunk: Chunk) {
    const { min, max } = chunk;

    const [startX, , startZ] = min;
    const [endX, , endZ] = max;

    const locations: Coords3[] = [];

    for (let vx = startX; vx < endX; vx++) {
      for (let vz = startZ; vz < endZ; vz++) {
        const vy = chunk.getMaxHeight([vx, vz]);
        if (Mine.registry.isPlantable(chunk.getVoxel([vx, vy, vz])) && this.isTreeLocation(vx, vz)) {
          locations.push([vx, vy, vz]);
        }
      }
    }

    return locations;
  }

  generate(chunk: Chunk) {
    const locations = this.sample(chunk);
    const types = Mine.registry.getTypeMap(['trunk', 'leaves', 'leaves-orange']);

    const updates: VoxelUpdate[] = [];

    for (const location of locations) {
      const [vx, vy, vz] = location;
      const test = 0.4124;
      const test2 = 0.1424;
      const test3 = 0.241;
      const test4 = 0.53425;
      const height = Noise.perlin2(vx, vz, test4) > 0.06 ? 3 : 2;
      const bushHeight =
        Noise.perlin2(vx, vz, test) > 0.2 ? 8 : Noise.perlin2(vx, vz, test2) > 0.1 ? 5 : height === 3 ? 3 : 2;

      const type = Noise.perlin2(vx, vz, 0.005) > 0.1 ? types['leaves-orange'] : types['leaves'];

      for (let i = 0; i < height; i++) {
        updates.push({ voxel: [vx, vy + i, vz], type: types.trunk });
      }

      const [tbx, tby, tbz] = [vx, vy + height, vz];

      const bushSize = 1;
      const bushBigSize = 2;

      for (let j = 0; j <= bushHeight; j++) {
        const limit = (j % 3 === 1 || j % 3 === (height === 2 ? 0 : 2)) && j !== bushHeight ? bushBigSize : bushSize;
        for (let i = -limit; i <= limit; i++) {
          for (let k = -limit; k <= limit; k++) {
            const center = i === 0 && k === 0;
            const mf = center && j !== bushHeight ? types.trunk : type;
            if (Math.abs(i) === limit && Math.abs(k) === limit) continue;
            if (!center && Noise.fractalOctavePerlin3(vx + i, vy + j, vz + k, test3) > 0.4) continue;
            updates.push({ voxel: [tbx + i, tby + j, tbz + k], type: mf });
          }
        }
      }
    }

    return updates;
  }
}

export { Trees };