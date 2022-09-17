import ReactDOM from 'react-dom';
import { useState } from 'react';
import styles from '../styles/RegistrationModal.module.css';
import { RegistrationParameters, API_BASE_URL } from '../data';
import useStore from '../store';

export interface RegistrationModalProps {
  show: boolean;
  onClose: () => void;
  onSuccess: () => void;
}

const RegistrationModal: React.FC<RegistrationModalProps> = ({ show, onClose, onSuccess }: RegistrationModalProps) => {
  let [showPassword, setShowPassword] = useState(false);

  let [password, setPassword] = useState("");
  let [email, setEmail] = useState("");
  let [name, setName] = useState("");
  let [username, setUsername] = useState("");
  let [error, setError] = useState<string | null>(null);

  let store: any = useStore();
  
  const register = async (p: RegistrationParameters) => {
    return fetch(`${API_BASE_URL}/register`, {
      method: "POST",
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(p)
    }).then(res => res.json()).then(res => {
      let data = JSON.parse(res);
      console.log(data);

      if(data.error) {
        setError(data.error);
        return;
      }

      store.setToken(data.session_token);
      onSuccess();

      return null;

    })
  }


  const modal = show ?
    <div className={styles.modalOverlay}>
      <div className={styles.modal}>
        <h1>Register</h1>

        <input
          onChange={(e: any) => setName(e.target.value)}
          type="text"
          name="name"
          placeholder="Name"
        />

        <input
          onChange={(e: any) => setUsername(e.target.value)}
          type="text"
          name="username"
          placeholder="Username"
        />

        <input
          onChange={(e: any) => setEmail(e.target.value)}
          type="email"
          name="name"
          placeholder="Email"
        />

        <input
          onChange={(e: any) => setPassword(e.target.value)}
          type={showPassword ? "text" : "password"}
          name="name"
          placeholder="Password"
        />

        <button
          className={styles.showPasswordButton}
          onClick={() => setShowPassword(!showPassword) }
        >{showPassword ? "Hide password" : "Show Password"}</button>

        <button
          onClick={() => register({
            username: username,
            name: name,
            email: email,
            password: password
          })}
          className={styles.loginButton}
        >Register</button>
      </div>
    </div> : null;

  if (typeof window !== "undefined") {
    return ReactDOM.createPortal(
      modal,
      document.getElementById("modal-root")
    );
  } else {
    return null;
  }
}

export default RegistrationModal;
