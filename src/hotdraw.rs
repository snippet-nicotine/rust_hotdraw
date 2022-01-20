trait Id {

}

struct Dimension {
    width: u32,
    height: u32
}

trait Drawing {
    fn addFigure(figure: Figure);
    fn removeFigure(figure: Figure);
    fn getFigures() -> Vec(Figure);
    fn find(id: Id) -> Figure;
}

trait Figure {
    fn moveBy(x: u32, y: u32);
    fn center();
    fn getSize() -> Dimension;
    fn clone() -> Figure;
}

trait DrawingView {
    fn activate();
    fn deactivate();
    fn mouseDown();
    fn mouseDrag();
    fn mouseUp();
    fn mouseMove();
}

trait Tool {
    fn activate();
    fn deactivate();
    fn mouseDown();
    fn mouseDrag();
    fn mouseUp();
    fn mouseMove();
}