import { createContext } from 'pathfinder_web_canvas'

function debug(text) {
  console.debug(text)
}

function render() {
  let canvas = document.getElementById("app")
  let ctx = createContext(canvas)
  ctx.fillStyle = "green"
  ctx.fillRect(10, 10, 150, 100)
  ctx.lineWidth = 10
  ctx.pfFlush()
}

window.onload = function() {
  debug(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> BEGIN <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<")
  render()
}
