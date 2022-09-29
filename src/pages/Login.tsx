import { CssVarsProvider } from "@mui/joy";
import Button from "@mui/joy/Button";
import Link from "@mui/joy/Link";
import Sheet from "@mui/joy/Sheet";
import TextField from "@mui/joy/TextField";
import Typography from "@mui/joy/Typography";
import { Link as RouterLink } from 'react-router-dom';

function Login() {
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
          <Typography level="body2">Login to continue</Typography>
        </div>
        <TextField
          name="email"
          type="email"
          placeholder="johndoe@email.com"
          label="Email"
        />
        <TextField
          name="password"
          type="password"
          placeholder="password"
          label="Password"
        />
        <Button
          sx={{
            mt: 1,
          }}
        >
          Log in
        </Button>
        <Typography
          endDecorator={<Link component={RouterLink} to="/register">Register</Link>}
          fontSize="sm"
          sx={{ alignSelf: 'center' }}
        >
          Don't have an account?
        </Typography>
      </Sheet>
    </CssVarsProvider>
  );
}

export default Login;