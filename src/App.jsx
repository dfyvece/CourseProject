import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

import { ThemeProvider, createTheme } from "@mui/material/styles";

import Button from "@mui/material/Button";
import TextField from "@mui/material/TextField";
import Grid from "@mui/material/Grid";
import Paper from "@mui/material/Paper";
import Typography from "@mui/material/Typography";

import VideoPlayer from "./components/VideoPlayer";

function App() {
  const [resultMsg, setResultMsg] = useState("");
  const [query, setQuery] = useState("");
  const [videos, setVideos] = useState([]);

  const theme = createTheme({
    palette: {
      mode: "dark",
    },
  });

  async function search() {
    const result = await invoke("search", { query });
    setResultMsg(result.msg);
    setVideos(result.videos);
  }

  return (
    <ThemeProvider theme={theme}>
      <Paper
        style={{
          position: "absolute",
          left: 0,
          top: 0,
          width: "100vw",
          height: "100vh",
          padding: "2em",
        }}
        square
      >
        <Grid
          container
          spacing={2}
          justifyContent="center"
          alignItems="center"
          style={{ width: "100%", maxHeight: "100%" }}
        >
          <Grid item xs={6}>
            <TextField
              id="greet-input"
              variant="outlined"
              style={{ width: "100%" }}
              onChange={(e) => setQuery(e.currentTarget.value)}
              placeholder="Enter english word or phrase..."
            />
          </Grid>
          <Grid item xs={6}>
            <Button
              variant="contained"
              style={{ width: "100%", height: "4em" }}
              onClick={() => search()}
            >
              Find Content
            </Button>
          </Grid>

          <Grid item xs={12}>
            <Typography>{resultMsg}</Typography>
          </Grid>
          <Grid item xs={12}>
            <VideoPlayer videos={videos} />
          </Grid>
        </Grid>
      </Paper>
    </ThemeProvider>
  );
}

export default App;
