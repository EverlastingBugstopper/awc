const SPACE = "&nbsp;";

class GraphQLValidator {
  private input: Lazy<HTMLTextAreaElement>;
  private output: Lazy<HTMLElement>;

  constructor(inputID: string, outputID: string) {
    this.input = new Lazy(() => <HTMLTextAreaElement>document.getElementById(inputID));
    this.output = new Lazy(() => <HTMLElement>document.getElementById(outputID));
  }

  async start() {
    await this.validate();
    this.input.handle.addEventListener("keyup", async () => { await this.validate() });
  }

  async validate() {
    const graphql = this.input.handle.value.toString();
    const graphqlLines = graphql.split("\n");
    const output = await fetch('/', {
      method: "POST",
      body: graphql
    });
    const json = await output.json();
    let pretties: [string?] = [];
    const diagnostics = json["diagnostics"];
    if (diagnostics.length > 0) {
      for (const diagnostic of diagnostics) {
        // error | warning | advice
        const severity = diagnostic["severity"];
        let severityColor = "text-info";
        let severityEmoji = "💡";
        switch (severity) {
          case "warning": {
            severityColor = "text-warning"
            severityEmoji = "⚠️";
            break;
          }
          case "error": {
            severityColor = "text-error"
            severityEmoji = "❌";
            break;
          }
        }
  
        // start building up the diagnostic div
        let inner = `<div class="block">`;
  
        // apollo-compiler validation error
        const code = diagnostic["code"];
        inner += `<span class=${severityColor}>${code}</span>`
  
        // cannot find type `Result` in this document
        const message = diagnostic["message"];
        inner += `<br/><span class="text-content">${SPACE}${severityEmoji}${SPACE}${message}</span>`
  
        // now for the hard part,
        // we are taking the offset and length
        // and putting the diagnostic in the context
        // of the source
        const labels = diagnostic["labels"];
        for (const l of labels) {
          const labelSpan = l["span"];
  
          // 34
          let labelOffset = labelSpan["offset"];

          // 6
          const labelLength = labelSpan["length"];

          let lineIdx = 0;
          let inline: [{maybeLineIdx: number, maybeHighlightSpace: string, maybeLine: string}?] = [];
          let lastLine = false;
          for (let graphqlLine of graphqlLines) {
            let maybeHighlightSpace = "";
            lineIdx += 1;
            let maybeLine = ""
            if (labelOffset > 0 || lastLine) {
              maybeLine += `<br/><span class="secondary-content">${SPACE}${lineIdx}${SPACE}|</span><span class="primary-content">${SPACE}${SPACE}`;
              for (let i = 0; i < graphqlLine.length; i++) {
                maybeLine += graphqlLine.charAt(i)
                labelOffset -= 1
                if (labelOffset > -2) {
                  maybeHighlightSpace += SPACE
                }
              }
              maybeLine += "</span>"
            }
            inline.push({maybeLine, maybeLineIdx: lineIdx, maybeHighlightSpace});
            if (labelOffset <= 0 && !lastLine) {
              lastLine = true
            } else if (lastLine) {
              break
            }
          }

          let realHighlightSpace = "";
          for (const maybeInline of inline.slice(-4)) {
            if (maybeInline) {
              const { maybeLineIdx, maybeHighlightSpace, maybeLine } = maybeInline
              inner += maybeLine
              if (maybeLineIdx == lineIdx - 1) {
                realHighlightSpace = maybeHighlightSpace
                let highlight = ""
                let labelSpace = ""
                for (let i = 0; i < labelLength; i++) {
                  highlight += "─"
                  if (i < (labelLength / 2)) {
                    labelSpace += SPACE
                  }
                }
                inner += `<br/><span class="text-info">${SPACE}${SPACE}${SPACE}·${realHighlightSpace}${highlight}</span>`
        
                // not found in this scope
                const label = l["label"];
                inner += `<br/><span class="text-info">${SPACE}${SPACE}${SPACE}·${realHighlightSpace}${labelSpace}╰──${label}</span>`
              }
            }
          }
        }

        const help = diagnostic["help"];
        if (help) {
          inner += `<br/>${SPACE}${SPACE}<span class="text-info">help:</span><span class="text-content">${SPACE}${help}</span>`
        }

        inner += "</div>"
        pretties.push(inner);
      }
      this.output.handle.innerHTML = `${pretties.join("")}\n${json["message"]}`;
    } else {
      this.output.handle.innerHTML = `<code class="text-success center">${json["pretty"]}</span>`
    }

  }
}

interface ILazyInitializer<T> {
  (): T
}

class Lazy<T> {
  private instance: T | null = null;
  private initializer: ILazyInitializer<T>;

  constructor(initializer: ILazyInitializer<T>) {
    this.initializer = initializer;
  }

  public get handle(): T {
    if (this.instance == null) {
      this.instance = this.initializer();
    }

    return this.instance;
  }
}

const load = async () => {
  const validator = new GraphQLValidator("graphql", "diagnostics");
  validator.start();
}

window.addEventListener('load', load)
