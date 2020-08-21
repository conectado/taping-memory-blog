export function set_marked_options(info){ 
  console.log(info);
  info.highlight = (code, lang) => {
    if(!!(lang && hljs.getLanguage(lang))) 
    {
      return hljs.highlight(lang,code).value;
    } 

    return hljs.highlightAuto(code).value;
  };

  marked.setOptions(info); 
}
