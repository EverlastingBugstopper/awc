let diagnosticHandle = null;
let graphqlHandle = null;

const validate = async () => {
  const graphql = graphqlHandle.value
    const result = await fetch('/validate', {
      method: "POST", 
      body: graphql
    });
    const json = await result.json();
    let pretties = "";
    for (const diagnostic of json["diagnostics"]) {
      pretties += diagnostic["pretty"].replaceAll("\n", "<br/>") + "<br/><br/>"
    }
    diagnosticHandle.innerHTML = pretties
}

window.addEventListener('load', () => {
  diagnosticHandle = document.getElementById("diagnostics");
  graphqlHandle = document.getElementById("graphql");
  validate()
})