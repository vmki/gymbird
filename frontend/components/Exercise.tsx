import styles from '../styles/Exercise.module.css';

interface ExerciseProps {
  name: string;
  image_name: string;
  id: string;
}

const Exercise: React.FC<ExerciseProps> = (props: ExerciseProps) => {
  return (
    <div className={styles.exercise}>
      <h1>{ props.name }</h1>
      <img src={`assets/${props.image_name}`} />
      <p>ID: {props.id}</p>
    </div>
  )
} 

export default Exercise;
