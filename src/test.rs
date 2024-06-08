use super::*;

#[test]
fn test_gen_map() {
    let board = Board {
        height : 3,
        width : 3,
        nums : vec![0; 3*3*3*3]
    };
    let map = board.gen_map();
    assert_eq!(map[0], [    
        1, 2, 3, 4, 5, 6, 7, 8,
        9, 10,11,
        18,19,20,
        27,
        36,
        45,
        54,
        63,
        72,
    ]);
    assert_eq!(map[30], [         
                 3,
                 12,
                 21,
        27,28,29,   31,32,33,34,35,
                 39,40,41,
                 48,49,50,
                 57,
                 66,
                 75,
    ]);
}

#[test]
fn test_is_solved() {
    let mut solved_board = Board {
        height : 3,
        width : 3,
        nums : vec![
            9,2,6,1,7,8,5,4,3,
            4,7,3,6,5,2,1,9,8,
            8,5,1,9,4,3,6,2,7,
            6,8,5,2,3,1,9,7,4,
            7,3,4,8,9,5,2,6,1,
            2,1,9,4,6,7,8,3,5,
            5,6,8,7,2,4,3,1,9,
            3,4,2,5,1,9,7,8,6,
            1,9,7,3,8,6,4,5,2,
        ]
    };
    assert_eq!(solved_board.is_solved(), true);
    solved_board.nums[12] = 1;
    assert_eq!(solved_board.is_solved(), false);
    solved_board.nums[12] = 0;
    assert_eq!(solved_board.is_solved(), false);
    solved_board.nums[12] = 10;
    assert_eq!(solved_board.is_solved(), false);
}

#[test]
fn test_solve_empty() {
    let board1 = Board {
        height : 4,
        width : 3,
        nums : vec![0; 3*3*4*4],
    };
    let solved1 = board1.solve();
    assert!(solved1.is_some());
    assert!(solved1.unwrap().is_solved());

    let board2 = Board {
        height : 2,
        width : 2,
        nums : vec![0; 2*2*2*2],
    };
    let solved2 = board2.solve();
    assert!(solved2.is_some());
    assert!(solved2.unwrap().is_solved());
}

#[test]
fn test_solve_partial() {
    let board = Board {
        height : 2,
        width : 3,
        nums : vec![
            1,0,4,0,3,6,
            0,0,6,1,0,0,
            0,4,1,0,2,5,
            3,0,2,4,0,0,
            2,0,3,0,0,4,
            4,0,5,2,0,3,
        ],
    };
    let solved = board.solve();
    assert!(solved.is_some());
    assert!(solved.unwrap().is_solved());
}
