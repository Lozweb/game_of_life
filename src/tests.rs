#[cfg(test)]

mod tests {
    use crate::univers::{Cell, Universe};
    use crate::univers::State::{Alive, Dead};

    #[test]
    fn create_universe(){
        let universe = Universe {
            universe: vec![
                vec![Cell{x:0, y:0, state: Alive}, Cell{x:0, y:1, state: Dead}, Cell{x:0, y:2, state: Dead}],
                vec![Cell{x:1, y:0, state: Dead}, Cell{x:1, y:1, state: Alive}, Cell{x:1, y:2, state: Alive}],
                vec![Cell{x:2, y:0, state: Alive}, Cell{x:2, y:1, state: Dead}, Cell{x:2, y:2, state: Dead}]
            ]
        };

        let nb_line = universe.universe.len();
        assert_eq!(nb_line, 3);

        for line in universe.universe {
            assert_eq!(line.len(), 3)
        }
    }

    #[test]
    fn find_voisin(){
        let universe = Universe {
            universe: vec![
                vec![Cell{x:0, y:0, state: Alive}, Cell{x:0, y:1, state: Dead}, Cell{x:0, y:2, state: Dead}],
                vec![Cell{x:1, y:0, state: Dead}, Cell{x:1, y:1, state: Alive}, Cell{x:1, y:2, state: Alive}],
                vec![Cell{x:2, y:0, state: Alive}, Cell{x:2, y:1, state: Dead}, Cell{x:2, y:2, state: Dead}]
            ]
        };

        let nb_voisin = Cell::find_voisin(&universe, 1, 1).len();
        assert_eq!(nb_voisin, 8)
    }

    #[test]
    fn find_voisin_bordure(){
        let universe = Universe {
            universe: vec![
                vec![Cell{x:0, y:0, state: Alive}, Cell{x:0, y:1, state: Dead}, Cell{x:0, y:2, state: Dead}],
                vec![Cell{x:1, y:0, state: Dead}, Cell{x:1, y:1, state: Alive}, Cell{x:1, y:2, state: Alive}],
                vec![Cell{x:2, y:0, state: Alive}, Cell{x:2, y:1, state: Dead}, Cell{x:2, y:2, state: Dead}]
            ]
        };

        let nb_voisin_corner_top_left = Cell::find_voisin(&universe, 0, 0).len();
        let nb_voisin_corner_bottom_right = Cell::find_voisin(&universe, 2, 2).len();
        let nb_voisin_corner_bottom_left = Cell::find_voisin(&universe, 2, 0).len();
        let nb_voisin_corner_top_right = Cell::find_voisin(&universe, 0, 2).len();

        assert_eq!(nb_voisin_corner_top_left, 3);
        assert_eq!(nb_voisin_corner_bottom_right, 3);
        assert_eq!(nb_voisin_corner_bottom_left, 3);
        assert_eq!(nb_voisin_corner_top_right, 3);
    }

    #[test]
    fn find_voisin_alive(){
        let universe = Universe {
            universe: vec![
                vec![Cell{x:0, y:0, state: Alive}, Cell{x:0, y:1, state: Dead}, Cell{x:0, y:2, state: Dead}],
                vec![Cell{x:1, y:0, state: Dead}, Cell{x:1, y:1, state: Alive}, Cell{x:1, y:2, state: Alive}],
                vec![Cell{x:2, y:0, state: Alive}, Cell{x:2, y:1, state: Dead}, Cell{x:2, y:2, state: Dead}]
            ]
        };

        let voisin_alive = Cell::voisin_by_state(&universe, 1, 1, Alive);

        assert_eq!(voisin_alive.len(), 3)
    }

    #[test]
    fn mut_cell_from_all_rules(){
        let universe = Universe {
            universe: vec![
                vec![Cell{x:0, y:0, state: Dead}, Cell{x:0, y:1, state: Alive}, Cell{x:0, y:2, state: Dead}],
                vec![Cell{x:1, y:0, state: Alive}, Cell{x:1, y:1, state: Alive}, Cell{x:1, y:2, state: Dead}],
                vec![Cell{x:2, y:0, state: Alive}, Cell{x:2, y:1, state: Dead}, Cell{x:2, y:2, state: Alive}]
            ]
        };

        let first_rule_cell_state= universe.apply_rules(0,0);
        let second_rule_cell_state = universe.apply_rules(1, 0);
        let third_rule_cell_state = universe.apply_rules(2,2);
        let fourth_rule_cell_state = universe.apply_rules(1,1);


        assert_eq!(first_rule_cell_state, Alive);
        assert_eq!(second_rule_cell_state, Alive);
        assert_eq!(third_rule_cell_state, Dead);
        assert_eq!(fourth_rule_cell_state, Dead);
    }

    #[test]
    fn calculate_universe_future(){
        let universe = Universe {
            universe: vec![
                vec![Cell{x:0, y:0, state: Dead}, Cell{x:0, y:1, state: Alive}, Cell{x:0, y:2, state: Dead}],
                vec![Cell{x:1, y:0, state: Alive}, Cell{x:1, y:1, state: Alive}, Cell{x:1, y:2, state: Dead}],
                vec![Cell{x:2, y:0, state: Alive}, Cell{x:2, y:1, state: Dead}, Cell{x:2, y:2, state: Alive}]
            ]
        };

        let universe_future = Universe{
            universe: vec![
                vec![Cell{x:0, y:0, state: Alive}, Cell{x:0, y:1, state: Alive}, Cell{x:0, y:2, state: Dead}],
                vec![Cell{x:1, y:0, state: Alive}, Cell{x:1, y:1, state: Dead}, Cell{x:1, y:2, state: Alive}],
                vec![Cell{x:2, y:0, state: Alive}, Cell{x:2, y:1, state: Dead}, Cell{x:2, y:2, state: Dead}]
            ]
        };

        assert_eq!(universe.calculate_future(), universe_future);
    }
}
