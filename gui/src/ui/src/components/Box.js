import React from 'react';
import Draggable from 'react-draggable';

export default class Box extends React.Component {
  render () {
    return (
      <Draggable cancel=".connector" onDrag={this.props.onDrag}>
        <div className="box">
          <div className="connector" data-id={this.props.id} />
          <div className="content"> {this.props.children}</div>
        </div>
      </Draggable>
    );
  }
}
