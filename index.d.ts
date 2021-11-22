type Element = {
    type: string,
    attributes:{
        [k:string]:string
    },
    children: Array<Element>
}

// parse svg text to ast object
export const parse: (input: String) => Element
// parse ast object to svg text 
export const stringify: (element: Element) => String