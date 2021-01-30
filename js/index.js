import { createContext } from '../pathfinder/web_canvas/pkg/pathfinder_web_canvas.js'

function debug(text) {
  console.debug(text)
}

function render() {
  let canvas = document.createElement("canvas")
  createContext(canvas)

  let app = document.getElementById("app")
  app.appendChild(canvas)
}

window.onload = function() {
  debug(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> BEGIN <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<")
  render()
}
