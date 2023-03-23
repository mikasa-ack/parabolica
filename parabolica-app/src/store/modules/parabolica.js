import { ApiPromise, WsProvider } from "@polkadot/api";
import { ContractPromise } from "@polkadot/api-contract";
import { Keyring } from "@polkadot/keyring";
import { BN, BN_ONE } from "@polkadot/util";

const MAX_CALL_WEIGHT = new BN(5_000_000_000_000).isub(BN_ONE);
const PROOFSIZE = new BN(1_000_000);
const wsProvider = new WsProvider('ws://127.0.0.1:9944');
const metadata = require("@/assets/parabolica.json");
const contract_addr = "5G6Kp8kt8t49K9C63K1nntaBkCmnC5txgYnGxot8uvpU3BUm";

const state = {
    track: null,
    lap_number: null,
    racer_positions: null,
};

const getters = {
    track: (state) => state.track,
    lap_number: (state) => state.lap_number,
    racer_positions: (state) => state.racer_positions,
};

const actions = {
    async fetchTrack({commit}) {
        let api = await ApiPromise.create({provider: wsProvider});
        let contract = await new ContractPromise(api, metadata, contract_addr);
        const keyring = new Keyring({ type: 'sr25519' });
        const alicePair = keyring.addFromUri('//Alice', { name: 'Alice default' });
        console.log("Contract: ", contract);
        let raw = await contract.query.getTrack(
            alicePair.address,
            {
            gasLimit: api.registry.createType('WeightV2', {
                refTime: MAX_CALL_WEIGHT,
                proofSize: PROOFSIZE,
            }),
            storageDepositLimit: undefined,
        });
        let track = await raw.output.toJSON()["ok"];
        console.log("TRACK: ", track);
        commit("setTrack", track);
    },
    async fetchLap({commit}) {
        let api = await ApiPromise.create({provider: wsProvider});
        let contract = await new ContractPromise(api, metadata, contract_addr);
        const keyring = new Keyring({ type: 'sr25519' });
        const alicePair = keyring.addFromUri('//Alice', { name: 'Alice default' });
        console.log("Contract: ", contract);
        let raw = await contract.query.getCurrLap(
            alicePair.address,
            {
            gasLimit: api.registry.createType('WeightV2', {
                refTime: MAX_CALL_WEIGHT,
                proofSize: PROOFSIZE,
            }),
            storageDepositLimit: undefined,
        });
        let lap = await raw.output.toJSON()["ok"];
        console.log("Lap: ", lap);
        commit("setLapNumber", lap);
    },
    async fetchRacerPositions({commit}) {
        let api = await ApiPromise.create({provider: wsProvider});
        let contract = await new ContractPromise(api, metadata, contract_addr);
        const keyring = new Keyring({ type: 'sr25519' });
        const alicePair = keyring.addFromUri('//Alice', { name: 'Alice default' });
        console.log("Contract: ", contract);
        let raw = await contract.query.getPositions(
            alicePair.address,
            {
            gasLimit: api.registry.createType('WeightV2', {
                refTime: MAX_CALL_WEIGHT,
                proofSize: PROOFSIZE,
            }),
            storageDepositLimit: undefined,
        });
        let positions = await raw.output.toJSON()["ok"];
        commit("setRacerPositions", positions.sort().reverse());
    }
};

const mutations = {
    setTrack(state, track) {
        state.track = track;
    },
    setLapNumber(state, lap) {
        state.lap_number = lap;
    },
    setRacerPositions(state, positions) {
        state.racer_positions = positions;
    },
};

export default {
    namespaced: true,
    state,
    getters,
    actions,
    mutations,
};