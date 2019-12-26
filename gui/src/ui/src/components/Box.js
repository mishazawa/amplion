import React from 'react';
import Draggable from 'react-draggable';

export default class Box extends React.Component {
  render () {
    return (
      <Draggable cancel=".connector">
        <div className="box">
          <div className="connector"/>
          <div className="content"> {this.props.children}</div>
        </div>
      </Draggable>
    );
  }
}
