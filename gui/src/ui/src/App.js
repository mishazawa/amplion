import React from 'react';

import Box from './components/Box';
import { connect } from 'react-redux';
import { setWires, removeWires } from './store/reducers/wires';
import { addBox, removeBox, applyModule } from './store/reducers/boxes';

import * as Native from './native';

const mapStateToProps = ({boxes, wires}) => ({boxes: boxes.elements, wires: wires.elements});

const mapDispatchToProps = dispatch => ({
  setWires   : val => dispatch(setWires(val)),
  addBox     : val => dispatch(addBox(val)),
  removeBox  : val => dispatch(removeBox(val)),
  applyModule: val => dispatch(applyModule(val)),
  removeWires: val => dispatch(removeWires(val)),
});


class App extends React.Component {
  state = {
    wireStart: null,
    wireEnd: null,
  }

  canvasRef = React.createRef();
  workspaceRef = React.createRef();
  canvasTopRef = React.createRef();

  addComponent = (attrs) => {
    this.props.addBox({ id: `box_${Math.random()}_${Math.random()}`, ...attrs })
  }

  componentDidMount () {
    window.addEventListener('keyup', this.onHotkey);
    window.addEventListener('mousemove', this.onMousemove);
    window.addEventListener('mousedown', this.onMousedown);
    window.addEventListener('mouseup', this.onMouseup);
    const {width, height} = this.workspaceRef.current.getBoundingClientRect();
    this.canvasRef.current.width = width;
    this.canvasRef.current.height = height;
    this.canvasTopRef.current.width = width;
    this.canvasTopRef.current.height = height;
    Native.init();
  }

  componentDidUpdate () {
    this.onDrag();
  }

  componentWillUnmount () {
    window.removeEventListener('keyup', this.onHotkey);
    window.removeEventListener('mousemove', this.onMousemove)
    window.removeEventListener('mousedown', this.onMousedown)
    window.removeEventListener('mouseup', this.onMouseup)
  }

  onHotkey = (e) => {
    if (e.ctrlKey && e.keyCode === 69) {
      this.addComponent({defaultPosition: {x: e.pageX, y: e.pageY}});
    }
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
    if (wireStart.getAttribute('data-id')) return;
    this.setState({
      wireStart: null,
      wireEnd: null,
    });
  }

  maybeConnectWires = () => {
    const { wireStart, wireEnd } = this.state;

    if (wireStart !== wireEnd) {
      const el = { start: wireStart.getAttribute('data-id'), end: wireEnd.getAttribute('data-id') };
      this.props.setWires(el);
      Native.connectWire(el.start, el.end);
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
    if (!wireStart || !wireEnd) return el;
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
    return this.props.wires.map((el) => this.getBoxesForConnect(el));
  }

  onDragEnd = () => {
    this.props.removeWires(this.onDrag());
  }

  removeBox = (id) => {
    this.props.removeBox(id);
    Native.removeMod(id);
    this.onDragEnd();
  }

  applyModule = (val) => {
    this.props.applyModule(val);
    switch (val.module) {
      case 'sin':
      case '~':
        Native.addOsc(val.id, 'sine');
        break;
      case 'sq':
      case 'square':
        Native.addOsc(val.id, 'square');
        break;
      case 'out':
      case ')))':
        Native.addOut(val.id);
        break;
      default:
        Native.addFreq(val.id, parseFloat(val.module));
    }
  }

  render () {
    return (
      <div className="workspace" ref={this.workspaceRef}>
        <div className="controls">
          <button onClick={() => this.addComponent()}>Add</button>
        </div>
        <div className="elements">
          {
            this.props.boxes.map(
              (el) => <Box {...el}
                          key={ el.id }
                          removeBox = {() => this.removeBox(el.id)}
                          applyModule = {(module) => this.applyModule({...el, module})}
                          onDrag    = {this.onDrag}
                          onDragEnd = {this.onDragEnd}>
                      </Box>
            )
          }
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
