import * as THREE from 'three';

import { TrackballControls } from 'three/addons/controls/TrackballControls.js';
import { CSS3DRenderer, CSS3DObject } from 'three/addons/renderers/CSS3DRenderer.js';

const camera = new THREE.PerspectiveCamera( 40, window.innerWidth / window.innerHeight, 1, 10000 );
camera.position.z = 3000;

const scene = new THREE.Scene();

const options:string[] = [
    "splash-screen",
    "1dcell",
    "8ceattourny_d1",
    "8ceattourny_d2",
    "8ceattourny_d3",
    "BadKaiJuJu",
    "br8kout",
    "carbon8",
    "caveexplorer",
    "chipquarium",
    "chipwar",
    "danm8ku",
    "down8",
    "flightrunner",
    "fuse",
    "ghostEscape",
    "glitchGhost",
    "horseWorldOnline",
    "knumberknower",
    "masquer8",
    "mastermind", 
    "mini-lights-out",
    "octoachip8story",
    "octojam1title",
    "octojam2title",
    "octojam3title",
    "octojam4title",
    "octojam5title",
    "octojam6title",
    "octojam7title",
    "octojam8title",
    "octojam9title",
    "octojam10title",
    "octorancher",
    "outlaw",
    "petdog",
    "piper",
    "pumpkindressup",
    "RPS",
    "slipperyslope",
    "snek", 
    "spacejam",
    "spaceracer",
    "spockpaperscissors",
    "superpong",
    "tank",
    "tombstontipp",
    "wdl"
];


let programSelected:HTMLDivElement;
let nameProgramSelected:string = "";
let objects: CSS3DObject[] = [];

let renderer = new CSS3DRenderer();
renderer.setSize( window.innerWidth, window.innerHeight );
let container = document.getElementById( 'container' )!
container.appendChild( renderer.domElement );


let controls = new TrackballControls( camera, renderer.domElement );
controls.minDistance = 800;
controls.maxDistance = 10000;
controls.addEventListener( 'change', render );

const bkgColorSelected = '#ffbe0b';
const bkgColor = "#ff5100e5"

for (let index = 0; index < options.length; index++) {
    const program = document.createElement('div');
    program.className = 'program';
    program.style.backgroundColor = bkgColor;
    program.addEventListener('pointerdown', ()=>{
        if(programSelected != null){
            programSelected.style.backgroundColor = bkgColor;
        }
        program.style.backgroundColor = bkgColorSelected
        programSelected = program;
        nameProgramSelected = options[index];
    })
    
    const name = document.createElement('div');
    name.className = 'name';
    name.textContent = options[index];
    program.appendChild(name);

    const objectCSS = new CSS3DObject(program);
    objectCSS.name = options[index];

    objects.push( objectCSS );
}

const nbRows = Math.ceil(Math.sqrt(options.length))
const nbColumns = Math.ceil(options.length/nbRows) - 1;
const width = 650;
const height = 200;

for(let column = 0; column < nbColumns; column++){
    for (let row = 0; row < nbRows; row++) {
        const indexOfObjects  = column+(nbRows*row); // transforms rows and collumn number into an index for an array
        const object = objects[indexOfObjects];
        object.position.x = (column*width)-1600
        object.position.y = row*height -500
        object.position.z = 0;
        scene.add(object)
    }
}


// go to emulator
const goBtn = document.createElement('div');
goBtn.className = "goBtn";
const btnTxt = document.createElement('div');
btnTxt.className = 'name';
btnTxt.textContent = "Go To Emulator";
goBtn.appendChild(btnTxt);
goBtn.addEventListener('pointerdown',()=>{
    if(nameProgramSelected != ""){
        console.log(nameProgramSelected)
        window.sessionStorage.setItem("program",nameProgramSelected);
        window.location.href = '/main/index.html'
    }
    btnTxt.textContent = "Nothing \nselected";
    btnTxt.style.fontSize = '50px'
    window.setTimeout(()=>{ 
        btnTxt.textContent = "Go To Emulator";
        btnTxt.style.fontSize = '60px'
    },800)
})
const goBtn3D = new CSS3DObject(goBtn);
goBtn3D.position.z = 0;
goBtn3D.position.y = -800;
goBtn3D.position.x = 0;
scene.add(goBtn3D)



render()


function render() {

    renderer.render( scene, camera );

}

function onWindowResize() {

    camera.aspect = window.innerWidth / window.innerHeight;
    camera.updateProjectionMatrix();

    renderer.setSize( window.innerWidth, window.innerHeight );

    render();

}

function animate() {

    requestAnimationFrame( animate );

    controls.update();

}




window.addEventListener( 'resize', onWindowResize );
animate();