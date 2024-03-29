slint::slint!{
    import {Button,VerticalBox, HorizontalBox} from "std-widgets.slint" ;
    component App inherits Window{
        
    VerticalBox {
        HorizontalBox {
            Image {
                source: @image-url("/Users/jd/dev/consulting/rust_gui2/rust_gui2/src/logo.png");
            width: 200px;
            height: 80px;
            }
            Button {    text:"Topic 1" ; }
            Button {    text:"Topic 2" ; }
            Button {    text:"Topic 3" ; }
            Button {    text:"Topic 4" ; }

        }
        HorizontalBox {
            Button {    text:"Section 1" ; }
            Button {    text:"Section 2" ; }
            Button {    text:"Section 3" ; }
            Button {    text:"Section 4" ; }
        }
        HorizontalBox {
            VerticalBox{
                Button {    text:"Section 1" ; }
            Button {    text:"Section 2" ; }
            Button {    text:"Section 3" ; }
            Button {    text:"Section 4" ; }
            Button {    text:"Section 5" ; }
            Button {    text:"Section 6" ; }
            Button {    text:"Section 7" ; }
            Button {    text:"Section 8" ; }
            Button {    text:"Section 9" ; }
            Button {    text:"Section 10" ; }
            Button {    text:"Section 11" ; }
            Button {    text:"Section 12" ; }
            Button {    text:"Section 13" ; }
            Button {    text:"Section 14" ; }
            Button {    text:"Section 15" ; }

            }
            VerticalBox {
                Image{
       source: @image-url("/Users/jd/dev/consulting/rust_gui2/rust_gui2/src/image.png");
            width: 400px;
            height: 300px;
                }
                Text {
                    text: "The urban sprawl covers Europe in the west and Asia to the east, geographical facts giving rise to the romantically evocative but somewhat fanciful description of Istanbul as a city that straddles two continents. Technically it does, but the Bosphorus isn’t the city’s only waterway. Just before it meets the Sea of Marmara, the Golden Horn – known locally as Haliç – branches off northwest. It eventually peters out inland unlike the Sea of Marmara, which leads to the Aegean Sea via the narrow Dardanelles Strait. Day and night, tankers and container ships are visible along the horizon near the Princess Islands, patiently waiting their turn to pass along the shipping route.";            
                    width: 400px;
            }
            }
            VerticalBox{
                Button {    text:"Feature 1" ; }
                Button {    text:"Feature 2" ; }
                Button {    text:"Feature 3" ; }
                Button {    text:"Feature 4" ; }
                Button {    text:"Feature 5" ; }
                Button {    text:"Feature 6" ; }
                
            }
        }
    }
}
    }

fn main() {
    App::new().unwrap().run().unwrap();
    println!("Hello, world!");
}
