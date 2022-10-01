import { CssVarsProvider } from "@mui/joy";
import Typography from "@mui/joy/Typography";

const Dashboard = () => {
  return (
    <CssVarsProvider>
      <Typography level="h1" component="h2">
        Hi, this is dashboard.
      </Typography>
    </CssVarsProvider>
  );
};

export default Dashboard;
