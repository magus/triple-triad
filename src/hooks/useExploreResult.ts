import { useAppState } from 'src/core/AppStateContext';
import { useClientState } from 'src/core/ClientStateContext';

export function useExploreResult() {
  const [state] = useAppState();
  const [client_state] = useClientState();

  const explore_result_item = state.explore_result?.results[client_state.explore_result_index];
  const last_move = explore_result_item?.game.last_move;

  if (!explore_result_item || !last_move) {
    return null;
  }

  const score = explore_result_item.score;
  const [card, square] = last_move;

  return { card, square, score };
}
