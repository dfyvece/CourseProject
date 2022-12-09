import { useState, useEffect, useCallback } from "react";

import Grid from "@mui/material/Grid";
import Button from "@mui/material/Button";
import Typography from "@mui/material/Typography";

function VideoPlayer({ videos }) {
  const [current, setCurrent] = useState(0);
  const [height, setHeight] = useState(window.innerHeight);

  useEffect(() => {
    function onResize() {
      setHeight(window.innerHeight);
    }
    window.addEventListener("resize", onResize);
    return () => window.removeEventListener("resize", onResize);
  }, [setHeight]);

  useEffect(() => {
    setCurrent(0);
  }, [videos, setCurrent]);

  const onClickPrev = useCallback(() => {
    setCurrent((c) => Math.max(c - 1, 0));
  }, [setCurrent]);

  const onClickNext = useCallback(() => {
    setCurrent((c) => Math.min(c + 1, videos.length - 1));
  }, [videos, setCurrent]);

  if (videos === null || videos.length <= 0) {
    return <></>;
  }

  const isLeftMost = current <= 0;
  const isRightMost = current >= videos.length - 1;

  const lang = "zh";

  console.log(current);
  console.log(videos[current]);

  const start = Math.floor(videos[current].start - 1);

  return (
    <Grid container spacing={2}>
      <Grid item xs={12}>
        <Typography>
          Video {current + 1}/{videos.length}
        </Typography>
      </Grid>
      <Grid item xs={12}>
        <iframe
          id="player"
          type="text/html"
          width="100%"
          height={height * 0.5}
          src={`https://www.youtube.com/embed/${videos[current].id}?start=${start};enablejsapi=1&cc_load_policy=1&cc_lang_pref=${lang}&hl=${lang}&autoplay=1`}
          frameborder="0"
          allow="autoplay"
          allowFullScreen
        ></iframe>
      </Grid>
      <Grid item xs={6}>
        <Button
          variant="contained"
          color="primary"
          disabled={isLeftMost}
          style={{ width: "100%", height: "4em" }}
          onClick={onClickPrev}
        >
          Previous
        </Button>
      </Grid>
      <Grid item xs={6}>
        <Button
          variant="contained"
          color="secondary"
          disabled={isRightMost}
          style={{ width: "100%", height: "4em" }}
          onClick={onClickNext}
        >
          Next
        </Button>
      </Grid>
    </Grid>
  );
}

export default VideoPlayer;
