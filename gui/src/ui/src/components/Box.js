import React from 'react';
import Draggable from 'react-draggable';
import Mod from './Mod';

export default class Box extends React.Component {
  render () {
    return (
      <Draggable cancel={[".connector", ".content"]} onDrag={this.props.onDrag} onStop={this.props.onDragEnd}>
        <div className="box">
          <div className="connector" data-id={this.props.id} />
          <button className="remove" data-id={this.props.id} onClick={this.props.removeBox}/>
          <Mod applyModule = {this.props.applyModule}/>
        </div>
      </Draggable>
    );
  }
}
