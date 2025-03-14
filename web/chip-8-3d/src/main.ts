import './style.css'
import * as THREE from 'three'
import { OrbitControls } from 'three/addons/controls/OrbitControls.js'
import { GLTFLoader } from 'three/addons/loaders/GLTFLoader.js';

const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(60, window.innerWidth/ window.innerHeight, 0.1, 10000);
camera.position.z = 500;
const renderer = new THREE.WebGLRenderer()
const controls = new OrbitControls(camera, renderer.domElement)
camera.position.setZ(10);
const gltfLoader = new GLTFLoader();
controls.update();
// var screen = new THREE.Object3D<THREE.Object3DEventMap>
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
        console.log('we got here')
        if(child.material){
          console.log('we got here2')
          console.log(child.name)
          const videoTexture = new THREE.VideoTexture(video);
          videoTexture.colorSpace = THREE.SRGBColorSpace;
          videoTexture.flipY = false;
          child.material.map = videoTexture;
          child.material.needsUpdate = true
          child.position.y = 0.28;
          scene.add(child)
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

  scene.children.forEach((child)=>{
    
  })
  video.play()

  camera.lookAt(scene.position)
  renderer.autoClear =false


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

animate()


