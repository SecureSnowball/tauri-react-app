import { CssVarsProvider } from "@mui/joy";
import Button from "@mui/joy/Button";
import Link from "@mui/joy/Link";
import Sheet from "@mui/joy/Sheet";
import TextField from "@mui/joy/TextField";
import Typography from "@mui/joy/Typography";
import { invoke } from "@tauri-apps/api/tauri";
import { FormEvent, useState } from "react";
import { useDispatch } from "react-redux";
import { Link as RouterLink, useNavigate } from 'react-router-dom';
import { AuthResponse } from "../interfaces/authResponse.interface";
import { authenticate } from "../state/authSlice";

function Register() {
  const [name, setName] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const dispatch = useDispatch();
  const navigate = useNavigate();

  async function handleRegister(e: FormEvent) {
    e.preventDefault();
    const authResponse: AuthResponse = await invoke("register_command", {
      payload: {
        name,
        email,
        password
      }
    });
    dispatch(authenticate({
      token: authResponse.token,
      user: authResponse.payload,
    }))
    navigate("/dashboard")
  }
  
  return (
    <CssVarsProvider>
      <Sheet
        sx={{
          maxWidth: 400,
          mx: 'auto', // margin left & right
          my: 4, // margin top & botom
          py: 3, // padding top & bottom
          px: 2, // padding left & right
          display: 'flex',
          flexDirection: 'column',
          gap: 2,
          borderRadius: 'sm',
          boxShadow: 'md',
        }}
      >
        <div>
          <Typography level="h4" component="h1">
            <b>Welcome!</b>
          </Typography>
          <Typography level="body2">Register to continue</Typography>
        </div>
        <form onSubmit={handleRegister}>
          <TextField
            name="name"
            type="text"
            placeholder="John Doe"
            label="Name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
          <TextField
            name="email"
            type="email"
            placeholder="johndoe@email.com"
            label="Email"
            value={email}
            onChange={(e) => setEmail(e.target.value)}
          />
          <TextField
            name="password"
            type="password"
            placeholder="password"
            label="Password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
          <Button
            type="submit"
            sx={{
              mt: 1,
            }}
          >
            Register
          </Button>
        </form>
        <Typography
          endDecorator={<Link component={RouterLink} to="/">Login</Link>}
          fontSize="sm"
          sx={{ alignSelf: 'center' }}
        >
          Have an account?
        </Typography>
      </Sheet>
    </CssVarsProvider>
  );
}

export default Register;