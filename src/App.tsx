// import { BrowserRouter, Routes, Route} from "react-router-dom";
// import '@fontsource/roboto/300.css';
// import '@fontsource/roboto/400.css';
// import '@fontsource/roboto/500.css';
// import '@fontsource/roboto/700.css';
// import Home from './pages/Home';
// import About from './pages/About';
// import NavBar from './components/NavBar';
// import Drawer from './components/Drawer';
// import Container from '@mui/material/Container';
import { CssVarsProvider, useColorScheme } from '@mui/joy/styles';
import Button from '@mui/joy/Button';
import React from 'react';
import Login from './pages/Login';
import { BrowserRouter, Route, Routes } from 'react-router-dom';
// import Home from './pages/Home';
import About from './pages/About';
import Register from './pages/Register';

const ModeToggle = () => {
  const { mode, setMode } = useColorScheme();
  const [mounted, setMounted] = React.useState(false);

  // necessary for server-side rendering
  // because mode is undefined on the server
  React.useEffect(() => {
    setMounted(true);
  }, []);
  if (!mounted) {
    return null;
  }

  return (
    <Button
      variant="outlined"
      onClick={() => {
        if (mode === 'light') {
          setMode('dark');
        } else {
          setMode('light');
        }
      }}
    >
      {mode === 'light' ? 'Turn dark' : 'Turn light'}
    </Button>
  );
};


function App() {
  return (
    <BrowserRouter>
      <Routes>
          {/* <ModeToggle /> */}
          <Route path="/" element={<Login />} />
          <Route path="/register" element={<Register />} />
          {/* <Route path="/login" element={<Login />} />
          <Route path="/about" element={<About />} /> */}
      </Routes>
    </BrowserRouter>
  );
}

export default App;
