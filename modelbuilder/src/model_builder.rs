use ast::ast::*;

fn model_builder(stack: Vec<Rule>) {
    for rule in stack {
        rule_case(rule);
    }
}

fn rule_case(rule: Rule) {
    match rule {
        // Creations
        Rule::CreatePlayer(players) => create_player(players),
        Rule::CreateTeam(team, players) => create_team(team, players),
        Rule::CreateTurnorder(players) => create_turnorder(players),
        Rule::CreateTurnorderRandom(players) => create_turnorder_random(players),
        Rule::CreateLocationOnPlayerCollection(location, player_collection) => create_location_on_player_collection(location, player_collection),
        Rule::CreateLocationOnTeamCollection(location, team_collection) => create_location_on_team_collection(location, team_collection),
        Rule::CreateLocationOnTable(location) => create_location_on_table(location),
        Rule::CreateLocationCollectionOnPlayerCollection(location_collection, player_collection) =>
            create_location_collection_on_player_collection(location_collection, player_collection),
        Rule::CreateLocationCollectionOnTeamCollection(location_collection, team_collection) =>
            create_location_collection_on_team_collection(location_collection, team_collection),
        Rule::CreateLocationCollectionOnTable(location_collection) => create_location_collection_on_table(location_collection),
        Rule::CreateCardOnLocation(location, types) => create_card_on_location(location, types),
        Rule::CreateTokenOnLocation(int_expr, token, location) => create_token_on_location(int_expr, token, location),
        Rule::CreatePrecedence(precedence, on_key_prec) => create_precedence(precedence, on_key_prec),
        Rule::CreatePrecedencePairs(precedence, key_value_pair) => create_precedence_pairs(precedence, key_value_pair),
        Rule::CreateCombo(combo, filter) => create_combo(combo, filter),
        Rule::CreateMemoryIntPlayerCollection(memory, int_expr, player_collection) => 
            create_memory_int_player_collection(memory, int_expr, player_collection),
        Rule::CreateMemoryStringPlayerCollection(memory, string_expr, player_collection) => 
            create_memory_string_player_collection(memory, string_expr, player_collection),
        Rule::CreateMemoryIntTable(memory, int_expr) => create_memory_int_table(memory, int_expr),
        Rule::CreateMemoryStringTable(memory, string_expr) => create_memory_string_table(memory, string_expr),
        Rule::CreateMemoryPlayerCollection(memory, player_collection) => create_memory_player_collection(memory, player_collection),
        Rule::CreateMemoryTable(memory) => create_memory_table(memory),
        Rule::CreatePointMap(precedence, on_key_point) => create_point_map(precedence, on_key_point),
        Rule::CreatePointMapPairs(precedence, key_value_int) => create_point_map_pairs(precedence, key_value_int),

        // Actions
        Rule::FlipAction(cards, status) => flip_action(cards, status),
        Rule::ShuffleAction(cards) => shuffle_action(cards),
        Rule::PlayerOutOfStageAction(player_expr) => player_out_of_stage_action(player_expr),
        Rule::PlayerOutOfGameSuccAction(player_expr) => player_out_of_game_succ_action(player_expr),
        Rule::PlayerOutOfGameFailAction(player_expr) => player_out_of_game_fail_action(player_expr),
        Rule::PlayerCollectionOutOfStageAction(player_collection) => player_collection_out_of_stage_action(player_collection),
        Rule::PlayerCollectionOutOfGameSuccAction(player_collection) => player_collection_out_of_game_succ_action(player_collection),
        Rule::PlayerCollectionOutOfGameFailAction(player_collection) => player_collection_out_of_game_fail_action(player_collection),
        Rule::SetMemoryInt(memory, int_expr) => set_memory_int(memory, int_expr),
        Rule::SetMemoryString(memory, string_expr) => set_memory_string(memory, string_expr),
        Rule::SetMemoryCollection(memory, coll) => set_memory_collection(memory, coll),
        Rule::CycleAction(player_expr) => cycle_action(player_expr),
        Rule::BidAction(quantity) => bid_action(quantity),
        Rule::BidActionMemory(memory, quantity) => bid_action_memory(memory, quantity),
        Rule::EndTurn => end_turn(),
        Rule::EndStage => end_stage(),
        Rule::EndGameWithWinner(player_expr) => end_game_with_winner(player_expr),
        Rule::DemandCardPositionAction(card_position) => demand_card_position_action(card_position),
        Rule::DemandStringAction(string_expr) => demand_string_action(string_expr),
        Rule::DemandIntAction(int_expr) => demand_int_action(int_expr),

        // Move-Actions
        Rule::ClassicMove(classic_mv) => classic_move(classic_mv),
        Rule::DealMove(deal_mv) => deal_move(deal_mv),
        Rule::ExchangeMove(exchange_mv) => exchange_move(exchange_mv),
        Rule::TokenMove(token_mv) => token_move(token_mv),

        // Score + Winner Rule
        Rule::ScoreRule(score_r) => score_rule(score_r),
        Rule::WinnerRule(winner_r) => winner_rule(winner_r),
    }
}


// Creations

fn create_player(players: Vec<PlayerName>) {

}

fn create_team(team: TeamName, players: Vec<PlayerName>) {

}

fn create_turnorder(players: Vec<PlayerName>) {

}

fn create_turnorder_random(players: Vec<PlayerName>) {

}

fn create_location_on_player_collection(location: Location, player_collection: PlayerCollection) {

}

fn create_location_on_team_collection(location: Location, team_collection: TeamCollection) {

}

fn create_location_on_table(location: Location) {

}

fn create_location_collection_on_player_collection(location_collection: LocationCollection, player_collection: PlayerCollection) {

}

fn create_location_collection_on_team_collection(location_collection: LocationCollection, team_collection: TeamCollection) {

}

fn create_location_collection_on_table(location_collection: LocationCollection) {

}

fn create_card_on_location(location: Location, types: Types) {

}

fn create_token_on_location(int_expr: IntExpr, token: Token, location: Location) {

}

fn create_precedence(precedence: Precedence, on_key_prec: OnKeyPrec) {

}

fn create_precedence_pairs(precedence: Precedence, key_value_pair: KeyValuePairs) {

}

fn create_combo(combo: Combo, filter: FilterExpr) {

}

fn create_memory_int_player_collection(memory: Memory, int_expr: IntExpr, player_collection: PlayerCollection) {

}

fn create_memory_string_player_collection(memory: Memory, string_expr: StringExpr, player_collection: PlayerCollection) {

}

fn create_memory_int_table(memory: Memory, int_expr: IntExpr) {

}

fn create_memory_string_table(memory: Memory, string_expr: StringExpr) {

}

fn create_memory_player_collection(memory: Memory, player_collection: PlayerCollection) {

}

fn create_memory_table(memory: Memory) {

}

fn create_point_map(precedence: Precedence, on_key_point: OnKeyPoint) {

}

fn create_point_map_pairs(precedence: Precedence, key_value_int: KeyValueInt) {

}

// Actions

fn flip_action(cards: CardSet, status: Status) {

}

fn shuffle_action(cards: CardSet) {

}

fn player_out_of_stage_action(player_expr: PlayerExpr) {

}

fn player_out_of_game_succ_action(player_expr: PlayerExpr) {

}

fn player_out_of_game_fail_action(player_expr: PlayerExpr) {

}

fn player_collection_out_of_stage_action(player_collection: PlayerCollection) {

}

fn player_collection_out_of_game_succ_action(player_collection: PlayerCollection) {

}

fn player_collection_out_of_game_fail_action(player_collection: PlayerCollection) {

}

fn set_memory_int(memory: Memory, int_expr: IntExpr) {

}

fn set_memory_string(memory: Memory, string_expr: StringExpr) {

}

fn set_memory_collection(memory: Memory, coll: Collection) {

}

fn cycle_action(player_expr: PlayerExpr) {

}

fn bid_action(quantity: Quantity) {

}

fn bid_action_memory(memory: Memory, quantity: Quantity) {

}

fn end_turn() {

}

fn end_stage() {

}

fn end_game_with_winner(player_expr: PlayerExpr) {

}

fn demand_card_position_action(card_position: CardPosition) {

}

fn demand_string_action(string_expr: StringExpr) {

}

fn demand_int_action(int_expr: IntExpr) {

}

// Move-Actions

fn classic_move(classic_mv: ClassicMove) {

}

fn deal_move(deal_mv: DealMove) {

}

fn exchange_move(exchange_mv: ExchangeMove) {

}

fn token_move(token_mv: TokenMove) {

}

// Score + Winner Rule

fn score_rule(score_r: ScoreRule) {

}

fn winner_rule(winner_r: WinnerRule) {

}