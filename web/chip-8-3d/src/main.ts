import './style.css'
import * as THREE from 'three'
import { OrbitControls } from 'three/addons/controls/OrbitControls.js'
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';
import initialzeProgram, {Program} from '../wasm/chip_8.js'

// TODO: someohow be able to get keyboard input that looks cool
// YOU also need to be able to tell when a key is releasted.
// google chip 8 emulator to view the common keys 


// TODO: make a program selector with all the roms

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(60, window.innerWidth/ window.innerHeight, 0.1, 10000);
camera.position.z = 500;
const renderer = new THREE.WebGLRenderer()
const controls = new OrbitControls(camera, renderer.domElement)
camera.position.setZ(10);
const gltfLoader = new GLTFLoader();
controls.update();

const emulatorData = await initialzeProgram();
const emulator = Program.new();


//#region DataTexture
const width = 64;
const height = 32;
const size = width * height;
const data = new Uint8Array( 4 * size );
const color = new THREE.Color('rgb(222, 17, 17)' );

const r = Math.floor( color.r * 255 );
const g = Math.floor( color.g * 255 );
const b = Math.floor( color.b * 255 );

for ( let i = 0; i < size; i ++ ) {
	const stride = i * 4;
	data[ stride ] = r;
	data[ stride + 1 ] = g;
	data[ stride + 2 ] = b;
	data[ stride + 3 ] = 255;
}
//#endregion
gltfLoader.load(
  "./models/old_screen.gltf", //monitor inception in scketchfab
  function(gltf){
    const gltfScene = gltf.scene;
    gltfScene.children.forEach((child)=> {
      if(child.name == 'Monitor_inception'){
        child.position.y = -2;
        scene.add(child)
      }
    })
  }
);

const video = document.getElementById('video') as HTMLVideoElement;
gltfLoader.load("./models/old_screen.gltf", 
  function(gltf){
    const gtlfScene = gltf.scene;
    gtlfScene.children.forEach((child)=>{
      if(child instanceof THREE.Mesh){
        if(child.material){//name - screen
          // const videoTexture = new THREE.VideoTexture(video);
          // videoTexture.colorSpace = THREE.SRGBColorSpace;
          // videoTexture.flipY = false;
          // child.material.map = videoTexture;
          // child.material.needsUpdate = true

          // used the buffer to create a DataTexture
          const texture = new THREE.DataTexture( data, width, height );
          texture.needsUpdate = true;
          child.material.map = texture;
          child.material.needsUpdate = true;
          child.position.y = 0.28;
          scene.add(child)
          // screen = child;
        }
      }
    })
  }
);

const startButton = document.getElementById( 'startButton' );
startButton?.addEventListener('click', function(){
  init();
})

function init(){
  //#region Setup Ligths
  const topLigth = new THREE.DirectionalLight(0xFFFFFF, 3)
  topLigth.position.set(0.5,1,1).normalize()
  topLigth.castShadow = true;
  scene.add(topLigth);

  const ambientLigth = new THREE.AmbientLight(0xFFFFFF, 1)
  scene.add(ambientLigth)
  //#endregion

  const overlay = document.getElementById( 'overlay' )!;
	overlay.remove();

  // #region setup renderer, camera and controls
  const container = document.getElementById("container")
  renderer.setPixelRatio( window.devicePixelRatio);
  renderer.setSize(window.innerWidth, window.innerHeight);
  container?.appendChild(renderer.domElement)


  renderer.render(scene, camera)
  //#endregion

  // video.play()

  camera.lookAt(scene.position)
  renderer.autoClear =false
  changeTexture()
}


function animate(){
  requestAnimationFrame(animate)
  controls.update()
  renderer.clear()
  renderer.render(scene, camera)
}


window.addEventListener("resize", function(){
  camera.aspect = this.innerWidth/ this.innerHeight;
  camera.updateProjectionMatrix();
  renderer.setSize(this.innerWidth, this.innerHeight);
})

function changeTexture(){
  const color = new THREE.Color(random_rgba())
  const redValue = Math.floor( color.r * 255 );
  const greenValue = Math.floor( color.g * 255 );
  const blueValue = Math.floor( color.b * 255 );

  // const randomColor = Math.floor(Math.random() * 255)
  emulator.tick();
  emulator.timer_tick();
  const data = new Uint8Array(emulatorData.memory.buffer, emulator.get_display(), size * 4)
  // console.log(data)
  // for ( let i = 0; i < size; i ++ ) {
  //   const stride = i * 4;
  //   data[ stride ] = redValue;// color value for red
  //   data[ stride + 1 ] = greenValue; // color value for green
  //   data[ stride + 2 ] = blueValue; // color value for blue
  //   data[ stride + 3 ] = 255;
  // }

  
  const texture = new THREE.DataTexture( data, width, height );
  texture.needsUpdate = true;

  scene.children.forEach((child)=>{
    if(child.name == "screen" && child instanceof THREE.Mesh && child.material){
      child.material.needsUpdate = true;
          child.position.y = 0.28;
          scene.add(child)
      child.material.map = texture;
      child.material.needsUpdate = true;
    }
  })

}



function random_rgba() {
  var o = Math.round, r = Math.random, s = 255;
  return 'rgba(' + o(r()*s) + ',' + o(r()*s) + ',' + o(r()*s) + ',' + r().toFixed(1) + ')';
}

setInterval(changeTexture, 0)

animate()


