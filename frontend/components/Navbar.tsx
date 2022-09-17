import styles from '../styles/Navbar.module.css';
import useStore from '../store';

interface NavbarProps {
  onLogin: () => void;
  onRegister: () => void;
  onLogout: () => void;
}

const Navbar: React.FC<NavbarProps> = ({ onLogin, onRegister, onLogout })  => {
  let store = useStore();

  return (
    <div className={styles.navbar}>
      <p>Home</p>
      <button onClick={() => {
        console.log(store);
        onLogin()
      }}>Log in</button>
      <button onClick={onRegister}>Register</button>
      <button onClick={onLogout}>Log out</button>
    </div>
  )
}

export default Navbar;
