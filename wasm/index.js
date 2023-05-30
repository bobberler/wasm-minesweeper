import { Board } from "wasm-minesweeper";

const table = document.getElementById("minesweeper-board");
const flagCountElem = document.getElementById("flagcounter");

let board;
let width;
let height;
let firstMove = true;
let gameInProgress = true;
let numMines;


const updateMineCount = () =>{
    flagCountElem.innerHTML = "&#128163 " + (numMines - board.get_flags());
}

const renderBoard = () =>{
    let data = board.render();
    for(let x = 0; x < width; x++){
        for(let y = 0; y < height; y++){
            let char = data[y + x*height];
            let symbol;
            switch (char) {
                case 'n':
                    symbol = "&#11036";//"&#11035";
                    table.rows[x].cells[y].id = "notclear";
                    break;
                case '0':
                    symbol = "&#11036";
                    table.rows[x].cells[y].id = "clear";
                    break;
                case 'f':
                    symbol = "&#128681";
                    table.rows[x].cells[y].id = "flag";
                    break;
                case 'b':
                    symbol = "&#128163";
                    table.rows[x].cells[y].id = "bomb";
                    break;
                default:
                    symbol = char;
                    table.rows[x].cells[y].id = "num";
                    break;

            }
            table.rows[x].cells[y].innerHTML = symbol;
        }
    }
}

const createTable = () => {
    for(let x = 0; x < width; x++){
        let tr = table.insertRow();
        for(let y = 0; y < height; y++){
            let td = tr.insertCell();
            td.onclick = function() {
                    if (firstMove){
                        board.init(numMines, x, y);
                        firstMove = false;
                    }
                    gameInProgress = board.clear(x, y);
                    renderBoard();
            }
            td.oncontextmenu = function(e){
                e.preventDefault();
                if(!firstMove && gameInProgress){
                    board.flag(x, y);
                    renderBoard();
                    updateMineCount();
                }
            }
        }
    }
};

const newGame = (w, h, mines) =>{
    while(table.firstChild){
        table.removeChild(table.firstChild);
    }
    board = Board.new(w, h);
    width = board.width();
    height = board.height();
    firstMove = true;
    gameInProgress = true;
    numMines = mines;
    flagCountElem.innerHTML = "&#128163 " + numMines;
    createTable();
    renderBoard();
}

const btn8 = document.getElementById("8x8");
btn8.addEventListener("click", function () {
        newGame(8, 8, 10);
        table.style = "font-size: 1em;"
});

const btn16 = document.getElementById("16x16");
btn16.addEventListener("click", function () {
        newGame(16, 16, 40);
        table.style = "font-size: 0.7em;"
        
});

const btn30 = document.getElementById("30x16");
btn30.addEventListener("click", function () {
        newGame(30, 16, 100);
        table.style = "font-size: 0.7em;"
});

newGame(8, 8, 10);

