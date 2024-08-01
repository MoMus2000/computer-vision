pub fn to_pictures(){
    // ffmpeg -i output.mp4 -vf "fps=10,scale=426:240" output_frame_%04d.png
}

pub fn to_video(){
    // ffmpeg -framerate 10 -i ./output_frame_%04d.png -c:v libx264 -r 30 -pix_fmt yuv420p output.mp4
}