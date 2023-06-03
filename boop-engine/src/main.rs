fn main() {
  println!("Hello, world!");
}

#[derive(Copy, Clone, Debug)]
enum Color {
  Orange,
  Gray,
}

#[derive(Copy, Clone, Debug)]
enum Piece {
  Cat,
  Kitten,
}

#[derive(Copy, Clone, Debug)]
struct GamePiece {
  color: Color,
  piece: Piece,
}

#[derive(Copy, Clone, Debug)]
struct GameBoard {
  grid: [[Option<GamePiece>; 6]; 6],
  turn: Color,
}


impl GameBoard {
  fn new() -> GameBoard {
    GameBoard {
      grid: [[None; 6]; 6],
      turn: Color::Orange,
    }
  }

  fn place_piece(&mut self, x: usize, y: usize, piece: GamePiece) {
    self.grid[x][y] = Some(piece);
  }

  fn print(&self) {
    for row in self.grid.iter() {
      for cell in row.iter() {
        match cell {
          Some(piece) => match (piece.color, piece.piece) {
            (Color::Orange, Piece::Cat) => print!("A"),
            (Color::Orange, Piece::Kitten) => print!("a"),
            (Color::Gray, Piece::Cat) => print!("B"),
            (Color::Gray, Piece::Kitten) => print!("b"),
          },
          None => print!("."),
        }
      }
      println!();
    }
  }
}

