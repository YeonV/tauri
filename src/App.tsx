import logo from './logo.svg';
import './App.css';
import { appWindow } from '@tauri-apps/api/window';
import { Button } from '@mui/material';
import useStore from './store/store';

function App() {
  const bears = useStore(state => state.bears)
  const increase = useStore(state => state.increase)
  document.getElementById('titlebar-minimize')?.addEventListener('click', () => appWindow.minimize());
  document.getElementById('titlebar-maximize')?.addEventListener('click', () => appWindow.toggleMaximize());
  document.getElementById('titlebar-close')?.addEventListener('click', () => appWindow.close());
  return (
    <div className='App'>
      <header className='App-header'>
        <img src={logo} className='App-logo' alt='logo' />
        <p>Hello Tauri + Vite + React + Zustand + Immer + MUI!</p>
        <p>
          <Button variant={"contained"} onClick={() => increase(1)}>
            count is: {bears}
          </Button>
        </p>
      </header>
    </div>
  );
}

export default App;
