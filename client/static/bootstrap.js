import "./blog_style.scss";
//import styles from "./third-parties/bootstrap_scss/bootstrap.scss";

import("../pkg").then(module => {
  module.run_app();
});
