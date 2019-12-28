if (!window.external.invoke) {
  window.external.invoke = (...args) => console.log('mock', ...args);
}


const invoke = (...args) => window.external.invoke(JSON.stringify(...args));

export const init           = ()         => invoke({ cmd : 'init',  type: '',           value: '',   id: '' });
export const addFreq        = (id, freq) => invoke({ cmd: 'module', type: 'freq',       value: freq, id });
export const addOsc         = (id, wave) => invoke({ cmd: 'module', type: 'osc',        value: wave, id });
export const addOut         = (id)       => invoke({ cmd: 'module', type: 'out',        value: id,   id });
export const removeMod      = (id)       => invoke({ cmd: 'module', type: 'remove',     value: id,   id });
export const connectWire    = (f, t)     => invoke({ cmd: 'wire',   type: 'connect',    value: f,    id: t });
export const disconnectWire = (f, t)     => invoke({ cmd: 'wire',   type: 'disconnect', value: f,    id: t });

