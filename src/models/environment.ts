export default class Environment {
  private parent: Environment | null;
  private values: Map<string, string>;
  private scopeName: string;

  constructor(scopeName: string, parent?: Environment) {
    this.parent = parent || null;
    this.values = new Map();
    this.scopeName = scopeName;
  }

  define(name: string, value: string) {
    this.values.set(name, value);
  }

  get(name: string): string | undefined {
    if (this.values.has(name)) {
      return this.values.get(name);
    }

    if (this.parent) {
      return this.parent.get(name);
    }

    return undefined;
  }

  getAll(): Map<string, string> {
    const inherited = this.parent ? this.parent.getAll() : new Map();
    for (const [key, value] of this.values) {
      inherited.set(key, value);
    }
    return inherited;
  }

  extend(scopeName: string): Environment {
    return new Environment(scopeName, this);
  }

  scopeId(): string {
    return this.scopeName;
  }
}
