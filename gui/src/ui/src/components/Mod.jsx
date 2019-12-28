import React from 'react';
import cs from 'classnames';

const COMMAND_REGEXP = /(~|sin|out|\)\)\)|sq|square|freq\s[0-9]+\.[0.9]|[0-9]+\.[0.9])/g;

export default class Mod extends React.Component {
  ref = React.createRef();
  state = {
    editable: true,
    value: '',
  }

  onChange = (e) => {
    const command = e.target.innerText.trim();
    if (COMMAND_REGEXP.test(command)) {
      this.lockModule(command).then(this.applyModule);
    }
  }

  lockModule = (value) => new Promise((res) => {
    this.setState({
      editable: false,
      value
    }, () => {
      this.ref.current.innerHTML = this.state.value;
      return res(this.state.value)
    });
  })

  applyModule = (value) => {
    this.props.applyModule(value);
  }

  render () {
    return (
      <div contentEditable={this.state.editable}
           className={cs("content", { bold: !this.state.editable, valid: !this.state.editable })}
           onInput={this.onChange}
           ref = {this.ref}
           />
    );
  }
}
