const SPACE = "&nbsp;";

// Originally inspired by  David Walsh (https://davidwalsh.name/javascript-debounce-function)

// Returns a function, that, as long as it continues to be invoked, will not
// be triggered. The function will be called after it stops being called for
// `wait` milliseconds.
const debounce = (func, wait) => {
  let timeout;

  return function executedFunction(...args) {
    const later = () => {
      clearTimeout(timeout);
      func(...args);
    };

    clearTimeout(timeout);
    timeout = setTimeout(later, wait);
  };
};

class GraphQLValidator {
  private input: Lazy<HTMLTextAreaElement>;
  private output: Lazy<HTMLElement>;
  private inputLines: Lazy<HTMLTextAreaElement>;

  constructor(inputID: string, outputID: string, inputLinesID) {
    this.input = new Lazy(() => <HTMLTextAreaElement>document.getElementById(inputID));
    this.inputLines = new Lazy(() => <HTMLTextAreaElement>document.getElementById(inputLinesID));
    this.output = new Lazy(() => <HTMLElement>document.getElementById(outputID));
  }

  async start() {
    await this.validate();
    this.input.handle.addEventListener("keyup", async () => { debounce(await this.validate(), 600) });
    this.input.handle.addEventListener("scroll", async () => {
      this.inputLines.handle.scrollTop = this.input.handle.scrollTop;
      this.inputLines.handle.scrollLeft = this.input.handle.scrollLeft;
    })
  }

  async validate() {
    const graphql = this.input.handle.value.toString();
    const numLines = graphql.split("\n").length;
    this.inputLines.handle.value = Array.from(Array(numLines).keys()).map((k) => { return k + 1; }).join("\n");
    const output = await fetch('/', {
      method: "POST",
      body: graphql
    });
    const json = await output.json();
    this.output.handle.innerHTML = json["context"];
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
  const validator = new GraphQLValidator("graphql", "diagnostics", "lines");
  validator.start();
}

window.addEventListener('load', load)
