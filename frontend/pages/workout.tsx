import { NextPage } from 'next';
import Navbar from '../components/Navbar';
import useStore from '../store';
import { Set, Exercise, WorkoutExercise } from '../data';
import Router from 'next/router';
import { useEffect, useState } from 'react';
import { getExercise } from '../util';
import ExerciseSelectionModal from '../components/ExerciseSelectionModal';

const DEFAULT_SETS: Set[] = [
  { weight: 50, reps: 12 },
  { weight: 50, reps: 12 },
  { weight: 50, reps: 12 },
]

const Workout: NextPage = () => {
  let [showExerciseModal, setShowExerciseModal] = useState(false);
  let [exercises, setExercises] = useState<WorkoutExercise[]>([]);
  let store: any = useStore();

  useEffect(() => {
    if(store.sessionToken === null) {
      window.location.href = "/";
    }
  }, [])

  return (
    <div>
      <div id="modal-root" />

      <ExerciseSelectionModal onSelect={(ex: Exercise) => {
        setExercises([...exercises, { exercise: ex, sets: DEFAULT_SETS }]);
        setShowExerciseModal(false);
      }} onClose={() => setShowExerciseModal(false)} show={showExerciseModal} />

      { exercises.map((ex: WorkoutExercise) => (
        <div>
          <h1>{ex.exercise.name}</h1>
          { ex.sets.map(s => <p>weight: {s.weight} kg | {s.reps} reps</p>) }
        </div>
      )) }
      <button onClick={() => setShowExerciseModal(true)}>Add Exercise</button>
    </div>
  )
}

export default Workout;
