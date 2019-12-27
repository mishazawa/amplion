const SET_WIRES = 'SET_WIRES';

const initialState = {
  elements: [],
};

export const wireReducer = (state = initialState, action) => {
  switch (action.type) {
    case SET_WIRES:
      return Object.assign({}, state, { elements: state.elements.concat(action.value) });
    default:
      return state;
  }
};


export const setWires = value => ({
  type: SET_WIRES,
  value
});
