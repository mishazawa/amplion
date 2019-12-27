import React from 'react';

import Box from './components/Box';
import { connect } from 'react-redux';
import { setWires } from './store/reducers/wires';
import { addBox } from './store/reducers/boxes';

const mapStateToProps = ({boxes, wires}) => ({boxes: boxes.elements, wires: wires.elements});

const mapDispatchToProps = dispatch => ({
  setWires   : val => dispatch(setWires(val)),
  addBox     : val => dispatch(addBox(val)),
});


class App extends React.Component {
  state = {
    wireStart: null,
    wireEnd: null,
  }

  canvasRef = React.createRef();
  workspaceRef = React.createRef();
  canvasTopRef = React.createRef();

  addComponent = (type) => {
    this.props.addBox({ id: Math.random() })
  }

  componentDidMount () {
    window.addEventListener('mousemove', this.onMousemove);
    window.addEventListener('mousedown', this.onMousedown);
    window.addEventListener('mouseup', this.onMouseup);
    const {width, height} = this.workspaceRef.current.getBoundingClientRect();
    this.canvasRef.current.width = width;
    this.canvasRef.current.height = height;
    this.canvasTopRef.current.width = width;
    this.canvasTopRef.current.height = height;
  }

  componentDidUpdate () {
    this.onDrag();
  }

  componentWillUnmount () {
    window.removeEventListener('mousemove', this.onMousemove)
    window.removeEventListener('mousedown', this.onMousedown)
    window.removeEventListener('mouseup', this.onMouseup)
  }

  onMousemove = (e) => {
    const { wireStart } = this.state;
    if (wireStart) {
      const { x, y, width } = wireStart.getBoundingClientRect();
      this.withCanvas((context, canvas) => {
        context.clearRect(0, 0, canvas.width, canvas.height);
        this.drawLine(x + width / 2, y + width / 2, e.clientX, e.clientY, this.canvasTopRef)
      }, this.canvasTopRef);
    }
  }

  onMousedown = (e) => {
    if (!e.target.getAttribute('data-id')) return;
    this.setState({
      wireStart: e.target,
    }, this.checkConnector);
  }

  onMouseup = (e) => {
    if (!e.target.getAttribute('data-id')) {
      return this.setState({
        wireStart: null,
        wireEnd: null,
      }, this.maybeConnectWires);
    }
    this.setState({
      wireEnd: e.target,
    }, this.maybeConnectWires);
  }

  checkConnector = () => {
    const { wireStart } = this.state;
    if (!wireStart.getAttribute('data-id')) return;

    console.log('can draw line', wireStart.getAttribute('data-id'));
  }

  maybeConnectWires = () => {
    const { wireStart, wireEnd } = this.state;

    if (wireStart !== wireEnd) {
      this.props.setWires({ start:wireStart.getAttribute('data-id'), end: wireEnd.getAttribute('data-id')});
    }

    this.withCanvas((context, canvas) => {
      context.clearRect(0, 0, canvas.width, canvas.height);
    }, this.canvasTopRef);

    this.setState({
      wireStart: null,
      wireEnd: null,
    });
  }

  getBoxesForConnect = (el) => {
    const { wireStart, wireEnd } = findBoxes(el);
    if (!wireStart || !wireEnd) return;
    const ws = wireStart.getBoundingClientRect();
    const we = wireEnd.getBoundingClientRect();
    this.drawLine(ws.x, ws.y, we.x, we.y);
  }

  withCanvas = (fn, ref = this.canvasRef) => fn(ref.current.getContext('2d'), ref.current);

  drawLine = (x1, y1, x2, y2, ref = this.canvasRef) => {
    this.withCanvas((context) => {
      context.beginPath();
      context.moveTo(x1, y1);
      context.lineTo(x2, y2);
      context.stroke();
    }, ref);
  }

  onDrag = () => {
    this.withCanvas((context, canvas) => context.clearRect(0, 0, canvas.width, canvas.height));
    this.props.wires.map((el) => this.getBoxesForConnect(el))
  }

  render () {
    return (
      <div className="workspace" ref={this.workspaceRef}>
        <div className="controls">
          <button onClick={() => this.addComponent('boxes')}>Add</button>
        </div>
        <div className="elements">
          { this.props.boxes.map((el) => <Box key={ el.id } {...el} onDrag = {() => this.onDrag()}/>)}
        </div>
        <canvas className="canvas" id="canvas" ref={this.canvasRef}/>
        <canvas className="canvas" id="canvas-top" ref={this.canvasTopRef}/>
      </div>
    )
  }
}

export default connect(mapStateToProps, mapDispatchToProps)(App)


const findBoxes = ({start, end}) => {
  try {
    const wireStart = document.querySelector(`div[data-id="${start}"]`);
    const wireEnd = document.querySelector(`div[data-id="${end}"]`);
    return {
      wireStart,
      wireEnd
    }
  } catch (e) {
    return {
      wireStart: null,
      wireEnd: null
    }
  }
}
