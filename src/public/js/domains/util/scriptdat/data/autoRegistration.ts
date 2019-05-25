import LMObject from './LMObject';
import { LMWorld } from './Script';
import addObject from './addObject';
import F from './Flags';

export default function autoRegistration(worlds: ReadonlyArray<LMWorld>) {
    const gui = [new LMObject(157, 12288, 36864, 4, 4, 10000, F.GRAIL_GUI, []),];
    const mau = [new LMObject(157, 14336, 28672, 4, 4, 10000, F.GRAIL_MAU, []),];
    const sun = [new LMObject(157, 55296, 12288, 4, 4, 10000, F.GRAIL_SUN, []),];
    const spr = [new LMObject(157, 12288, 12288, 4, 4, 10000, F.GRAIL_SPR, []),];
    const inf = [new LMObject(157, 28672, 4096, 4, 4, 10000, F.GRAIL_INF, []),];
    const ext = [new LMObject(157, 2048, 12288, 4, 4, 10000, F.GRAIL_EXT, []),];
    const tlf = [new LMObject(157, 8192, 36864, 4, 4, 10000, F.GRAIL_TLF, []),];
    const end = [new LMObject(157, 43008, 4096, 4, 4, 10000, F.GRAIL_END, []),];
    const shr = [new LMObject(157, 40960, 4096, 4, 4, 10000, F.GRAIL_SHR, []),];
    const con = [new LMObject(157, 26624, 4096, 4, 4, 10000, F.GRAIL_CON, []),];
    const gra = [new LMObject(157, 14336, 12288, 4, 4, 10000, F.GRAIL_GRA, []),];
    const moo = [new LMObject(157, 20480, 20480, 4, 4, 10000, F.GRAIL_MOO, []),];
    const god = [new LMObject(157, 28672, 4096, 4, 4, 10000, F.GRAIL_GOD, []),];
    const rui = [new LMObject(157, 28672, 36864, 4, 4, 10000, F.GRAIL_RUI, []),];
    const bir = [new LMObject(157, 57344, 36864, 4, 4, 10000, F.GRAIL_BIR, []),];
    const tlb = [new LMObject(157, 49152, 36864, 4, 4, 10000, F.GRAIL_TLB, []),];
    const dim = [new LMObject(157, 16384, 20480, 4, 4, 10000, F.GRAIL_DIM, []),];

    worlds = addObject(worlds, 0, 2, 1, gui);
    worlds = addObject(worlds, 2, 0, 2, mau);
    worlds = addObject(worlds, 3, 2, 0, sun);
    worlds = addObject(worlds, 4, 1, 3, spr);
    worlds = addObject(worlds, 5, 2, 3, inf);
    worlds = addObject(worlds, 6, 3, 4, ext);
    worlds = addObject(worlds, 7, 0, 0, end);
    worlds = addObject(worlds, 8, 1, 4, shr);
    worlds = addObject(worlds, 9, 0, 0, tlf);
    worlds = addObject(worlds, 10, 3, 0, tlb);
    worlds = addObject(worlds, 11, 0, 1, con);
    worlds = addObject(worlds, 12, 3, 1, gra);
    worlds = addObject(worlds, 13, 0, 4, god);
    worlds = addObject(worlds, 14, 0, 1, moo); // not a typo
    worlds = addObject(worlds, 15, 0, 1, rui);
    worlds = addObject(worlds, 16, 3, 0, bir);
    worlds = addObject(worlds, 17, 2, 2, dim);

    return worlds;
}

