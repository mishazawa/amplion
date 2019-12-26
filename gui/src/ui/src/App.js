import React from 'react';
import Box from './components/Box';

export default class App extends React.Component {
  state = {
    boxes: []
  }
  addComponent = (type) => {
    this.setState((prevState) => ({
          [type]: prevState.boxes.concat({})
        }))
  }

  render () {
    return (
      <div className="workspace">
        <div className="controls">
          <button onClick={() => this.addComponent('boxes')}>Add</button>
        </div>
        <div className="elements">
        { this.state.boxes.map((el, k) => <Box key = {k}/>)}
        </div>
      </div>
    )
  }
}
