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
};

const getters = {
    track: (state) => state.track,
};

const actions = {
    async fetchTrack({commit}) {
        console.log("I WAS CALLED");
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
    }
};

const mutations = {
    setTrack(state, track) {
        state.track = track;
    },
};

export default {
    namespaced: true,
    state,
    getters,
    actions,
    mutations,
};