import './style.css'
import * as THREE from 'three'
import { OrbitControls } from 'three/addons/controls/OrbitControls.js'
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';
import initialzeProgram, {Program} from '../wasm/chip_8.js'
import { SceneNode } from 'three/webgpu';
import { getKeyframeOrder } from 'three/src/animation/AnimationUtils.js';
import { log } from 'three/tsl';
import { ConvexObjectBreaker } from 'three/examples/jsm/Addons.js';

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
const yValueOfScreen = 1.28;
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

// #region load models

//monitor
gltfLoader.load(
  "./models/old_screen.gltf", //monitor inception in scketchfab
  function(gltf){
    gltf.scene.children.forEach((child)=> {
      if(child.name == 'Monitor_inception'){
        child.position.y = -1;
        scene.add(child)
      }
    })
  }
);

// screen
// const video = document.getElementById('video') as HTMLVideoElement;
gltfLoader.load("./models/old_screen.gltf", 
  function(gltf){
    gltf.scene.children.forEach((child)=>{
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
          child.position.y = yValueOfScreen;
          scene.add(child)
          // screen = child;
        }
      }
    })
  }
);

//keyboard

//PAD
gltfLoader.load("./models/keyboard.gltf", 
  function(gltf){
    // gltf.scene.scale.set(1,1,1)
    // gltf.scene.position.z = 3.5;
    // gltf.scene.position.x = 0.25;
    // gltf.scene.position.y = -2;
    gltf.scene.children.forEach((child)=>{
      if(child instanceof THREE.Mesh && child.name == 'pad'){
        child.scale.multiplyScalar(0.1);
        child.position.z = 3.5;
        child.position.x = -0.05;
        child.position.y = -2;
        child.rotateX(1)
        scene.add(child)
      }
    })
  }
);


// let listOfKeys: [string,number][] = [["letterA", 0],["letterB",0],["letterC",0],["letterD",0],["letterE",0],
// ["letterF",0],["number0",0],["number1",0],["number2",0],["number3",0],["number4",0],["number5",0],
// ["number6",0],["number7",0],["number8",0],["number9",0]];

const nameOfKeys: string[] = ["letterA","letterB","letterC","letterD","letterE","letterF",
  "number0","number1","number2","number3","number4","number5","number6","number7","number8","number9"]
let isKeyPresent: number[] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]  

for (let index = 0; index < nameOfKeys.length; index++) {
  const keyName = nameOfKeys[index];
  gltfLoader.load("./models/keyboard.gltf", 
    function(gltf){
      gltf.scene.children.forEach(
        (child)=>{
          if(child.name == keyName){
            child.scale.multiplyScalar(0.1)
            child.position.divideScalar(10)
          if(child.name == "number1" || child.name == "number2" || child.name == "number3" || child.name == "letterC"){
            child.position.z += 4.4;
            child.position.y += -1.25;
          }
          if(child.name == "number4" || child.name == "number5" || child.name == "number6" || child.name == "letterD"){
            child.position.z += 4.15;
            child.position.y += -1.80;
          }
          if(child.name == "number7" || child.name == "number8" || child.name == "number9" || child.name == "letterE"){
            child.position.z += 3.85;
            child.position.y += -2.35;
          }
          if(child.name == "letterA" || child.name == "number0" || child.name == "letterB" || child.name == "letterF"){
            child.position.z += 3.55;
            child.position.y += -2.90;
          }
          child.position.x+=0.25;
          
          isKeyPresent[index] = 1;
          child.rotateX(1)
          scene.add(child);
        }
        }
      )
    }
  );
}

//#endregion


const startButton = document.getElementById( 'startButton' );
startButton?.addEventListener('click', ()=>init())

function init(){
  //#region Setup Ligths
  const topLigth = new THREE.DirectionalLight(0xFFFFFF, 2)
  topLigth.position.set(2,1,1).normalize()
  // topLigth.castShadow = true;
  scene.add(topLigth);

  const topLigth2 = new THREE.DirectionalLight(0xFFFFFF, 2)
  topLigth2.position.set(-2,1,1).normalize()
  // topLigth.castShadow = true;
  scene.add(topLigth2);
  //#endregion
  // scene.add(new THREE.DirectionalLightHelper(topLigth,5))
  // scene.add(new THREE.DirectionalLightHelper(topLigth2,5))

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


window.addEventListener("resize", 
  function(){
    camera.aspect = this.innerWidth/ this.innerHeight;
    camera.updateProjectionMatrix();
    renderer.setSize(this.innerWidth, this.innerHeight);
  }
);

function changeTexture(){
  emulator.tick();
  emulator.timer_tick();
  const data = new Uint8Array(emulatorData.memory.buffer, emulator.get_display(), size * 4)
  
  const texture = new THREE.DataTexture( data, width, height );
  texture.needsUpdate = true;

  scene.children.forEach((child)=>{
    if(child.name == "screen" && child instanceof THREE.Mesh && child.material){
      child.material.needsUpdate = true;
          child.position.y = yValueOfScreen;
          scene.add(child)
      child.material.map = texture;
      child.material.needsUpdate = true;
    }
  })

}

const zMovementKey = 0.15;
const yMovementKey = 0.05;
window.addEventListener('keydown', 
  (KeyboardEvent)=>{
    if(KeyboardEvent.repeat == false){
      const numberName = "number"+KeyboardEvent.key;
      const letterName = "letter"+KeyboardEvent.key.toUpperCase();
      if(nameOfKeys.includes(numberName) || nameOfKeys.includes(letterName)){
        scene.children.forEach((child)=>{
          if(child.name == numberName|| child.name == letterName){
            child.position.z -= zMovementKey;
            child.position.y -= yMovementKey;
          }
        })
      } 
    }
  }
)
window.addEventListener('keyup', 
  (KeyboardEvent)=>{
    const numberName = "number"+KeyboardEvent.key;
    const letterName = "letter"+KeyboardEvent.key.toUpperCase();
    if(nameOfKeys.includes(numberName) || nameOfKeys.includes(letterName)){
      scene.children.forEach((child)=>{
        if(child.name == numberName|| child.name == letterName){
          child.position.z += zMovementKey;
          child.position.y += yMovementKey;
        }
      })
    }
  }
)



setInterval(changeTexture, 0)

animate()


