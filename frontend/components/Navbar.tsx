import styles from '../styles/Navbar.module.css';
import useStore from '../store';

interface NavbarProps {
  onLogin: () => void;
  onRegister: () => void;
  onLogout: () => void;
  loggedIn: boolean;
}

const Navbar: React.FC<NavbarProps> = ({ onLogin, onRegister, onLogout, loggedIn })  => {
  let buttons = loggedIn ? (
    <div>
      <button onClick={() => {
        onLogin()
      }}>Log in</button>
      <button onClick={onRegister}>Register</button>
    </div>
  ) : (
    <div>
      <button onClick={onLogout}>Log out</button> 
    </div>
  )

  return (
    <div className={styles.navbar}>
      <p>Home</p>
      { buttons }
    </div>
  )
}

export default Navbar;
