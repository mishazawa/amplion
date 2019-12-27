import { combineReducers } from 'redux';
import { wireReducer } from './wires';
import { boxReducer } from './boxes';

export default combineReducers({
  wires: wireReducer,
  boxes: boxReducer
});
