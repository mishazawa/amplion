const SET_WIRES = 'SET_WIRES';
const RM_WIRES = 'RM_WIRES';

const initialState = {
  elements: [],
};

export const wireReducer = (state = initialState, action) => {
  switch (action.type) {
    case SET_WIRES:
      return Object.assign({}, state, { elements: state.elements.concat(action.value) });
    case RM_WIRES:
      return Object.assign({}, state, { elements: filterExistedWires(state.elements, action.value) });
    default:
      return state;
  }
};


export const setWires = value => ({
  type: SET_WIRES,
  value
});

export const removeWires = value => ({
  type: RM_WIRES,
  value
});

const filterExistedWires = (wires, toRemove) => wires.filter(elem => !toRemove.some(elem2 => elem === elem2))
