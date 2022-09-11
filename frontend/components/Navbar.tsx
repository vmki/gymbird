import styles from '../styles/Navbar.module.css';

interface NavbarProps {
  onLogin: () => void;
  onRegister: () => void;
}

const Navbar: React.FC<NavbarProps> = ({ onLogin, onRegister})  => {
  return (
    <div className={styles.navbar}>
      <p>Home</p>
      <button onClick={onLogin}>Log in</button>
      <button onClick={onRegister}>Register</button>
    </div>
  )
}

export default Navbar;
