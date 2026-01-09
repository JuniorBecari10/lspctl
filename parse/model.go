package parse

type ParsedYaml struct {
    Name string `yaml:"name"`
    Desc string `yaml:"description"`
    Homepage string `yaml:"homepage"`
    
    Licenses []string `yaml:"licenses"`
    Languages []string `yaml:"languages"`
    Categories []string `yaml:"categories"`

    Source rawSource `yaml:"source"`
    Bin map[string]string `yaml:"bin"`
}

// TODO: support source/extra_packages
type rawSource struct {
    Id string `yaml:"id"`
    ExtraPkgs []string `yaml:"extra_packages"` // do not make this nil.
    // TODO: add more
}

// ---

type InstallYaml struct {
    Name string
    Source Source
    BinName string
}

type parsedPurl struct {
    RawSource string
    Name string
    Version string
}

type PackageSource int
const (
    SourceGo PackageSource = iota
    SourceNpm
    SourceGitHub
    SourceGeneric
    SourceUnknown
)

type Source struct {
    PkgSource PackageSource
    Name string
    Version string
    ExtraPkgs []string // no more info because it's assumed to use the same source and no pinned version
}
