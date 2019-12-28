const ADD_BOX = 'ADD_BOX';
const RM_BOX = 'RM_BOX';
const APPLY_MOD = 'APPLY_MOD';

const initialState = {
  elements: [{id: 'first'}, {id: 'second'}]
};

export const boxReducer = (state = initialState, action) => {
  switch (action.type) {
    case ADD_BOX:
      return Object.assign({}, state, { elements: state.elements.concat(action.value) });
    case RM_BOX:
      return Object.assign({}, state, { elements: state.elements.filter(({id}) => action.value !== id) });
    case APPLY_MOD:
      return Object.assign({}, state, { elements: editElement(state.elements, action.value) });
    default:
      return state;
  }
};


export const addBox = value => ({
  type: ADD_BOX,
  value
});

export const removeBox = value => ({
  type: RM_BOX,
  value
});

export const applyModule = value => ({
  type: APPLY_MOD,
  value
});

const editElement = (elements, elem) => {
  const index = elements.findIndex(({id}) => id === elem.id);
  if (index !== null) {
    elements[index] = elem;
  }
  return elements;
}
