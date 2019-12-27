const ADD_BOX = 'ADD_BOX';

const initialState = {
  elements: [/*{id: 1}, {id: 2}*/]
};

export const boxReducer = (state = initialState, action) => {
  switch (action.type) {
    case ADD_BOX:
      return Object.assign({}, state, { elements: state.elements.concat(action.value) });
    default:
      return state;
  }
};


export const addBox = value => ({
  type: ADD_BOX,
  value
});
