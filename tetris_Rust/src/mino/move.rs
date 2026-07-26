
pub fn move_piece<const action: char>(mino: Mino) -> Mino {
   let mut new_mino = mino;
   match action {
      'l' => {
         new_mino.x = mino.x - 1;
         new_mino.state = mino.state << Simd::splat(1);
      }
      'r' => {
         new_mino.x = mino.x + 1;
         new_mino.state = mino.state >> Simd::splat(1);
      }
      'd' => {
         new_mino.state = new_mino.state.rotate_elements_right::<1>();
      }
      'L' => {
         new_mino.angle = (mino.angle + 1) % 4;
         new_mino = rebuild(new_mino);


      }
      'R' => {
         new_mino.angle = (mino.angle + 3) % 4;
         new_mino = rebuild(new_mino);
      }
      _ => {}
   }
   new_mino
}
