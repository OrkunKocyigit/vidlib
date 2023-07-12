export interface IFileInfo {
  path: string;
  name: string;
  depth: number;
  get displayName(): string;
}

export class FileInfo implements IFileInfo {
  depth: number;
  name: string;
  path: string;

  constructor(depth: number, name: string, path: string) {
    this.depth = depth;
    this.name = name;
    this.path = path;
  }

  get displayName(): string {
    if (this.depth === 0) {
      return this.path;
    }
    return this.name;
  }
}
